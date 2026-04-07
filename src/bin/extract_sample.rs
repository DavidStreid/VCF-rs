use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: cargo run --bin extract_sample <PATH> <SAMPLE>");
        std::process::exit(1);
    }

    let path = &args[1];
    let target_sample = &args[2]; 
    
    println!("Opening file: {}", path);
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut sample_index: Option<usize> = None;
    let mut record_count = 0;

    for line in reader.lines() {
        let line = line?;

        // 1. Metadata lines
        if line.starts_with("##") {
            continue; 
        }

        // 2. The Header line
        if line.starts_with('#') {
            let header_parts: Vec<&str> = line.split('\t').collect();
            sample_index = header_parts.iter().position(|&name| name == target_sample);
            match sample_index {
                Some(idx) => {},
                None => {
                    eprintln!("ERROR: Could not find '{}' in the header columns.", target_sample);
                    return Ok(());
                }
            }
            continue;
        }

        // 3. Data Rows
        if let Some(idx) = sample_index {
            let fields: Vec<&str> = line.split('\t').collect();
            
            if let (Some(chrom), Some(pos), Some(sample_data)) = (fields.get(0), fields.get(1), fields.get(idx)) {
                 println!("{}\t{}\t{}", chrom, pos, sample_data);
                 record_count += 1;
            }
        }
    }

    println!("Processed {} total records.", record_count);
    Ok(())
}
