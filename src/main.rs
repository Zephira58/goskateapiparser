// src/main.rs

mod items;
mod parser;

use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let mut is_verbose = false;

    if args.contains(&String::from("-v")) || args.contains(&String::from("--verbose")) {
        is_verbose = true;
    }

    // Now correctly passing the `is_verbose` argument
    let yaml_output = parser::run_trade_analysis("src\\data\\tradexport_1755362248.csv", is_verbose)?;
    println!("{}", yaml_output);
    Ok(())
}
