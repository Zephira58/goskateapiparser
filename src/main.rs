mod items;
mod parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let yaml_output = parser::run_trade_analysis("src\\data\\tradexport_1755362248.csv")?;
    println!("{}", yaml_output);
    Ok(())
}
