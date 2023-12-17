use day3::sum_gear_ratios;
use std::env;
use std::fs;

fn main() -> Result<(), String> {
    let args = env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        return Err(String::from("invalid arguments"));
    }

    println!(
        "{}",
        sum_gear_ratios(&fs::read_to_string(&args[1]).expect("should not be empty"))
    );
    Ok(())
}
