// src/main.rs

mod items;
mod parser;

use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let mut is_verbose = false;
    let mut file_path = "".to_string(); // Default file path

    // Iterate through arguments to find flags and their values
    let mut i = 0;
    while i < args.len() {
        if args[i] == "-v" || args[i] == "--verbose" {
            is_verbose = true;
        } else if args[i] == "-d" || args[i] == "--data" {
            // Check if there's a next argument for the file path
            if let Some(path) = args.get(i + 1) {
                file_path = path.clone();
                i += 1; // Skip the next argument as it's the file path
            } else {
                eprintln!("Error: -d or --data flag requires a file path.");
                return Err("Missing file path for -d flag".into());
            }
        }
        i += 1;
    }

    // Pass the dynamic file_path and is_verbose flag
    let yaml_output = parser::run_trade_analysis(&file_path, is_verbose)?;
    println!("{}", yaml_output);
    Ok(())
}
