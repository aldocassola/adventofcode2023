use std::{env, fs};

use day3::sum_parts;

fn main() -> Result<(), &'static str> {
    let args = env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        return Err("invalid arguments");
    }

    let input = fs::read_to_string(&args[1]).expect("should not be empty");

    println!("{}", sum_parts(&input));
    Ok(())
}
