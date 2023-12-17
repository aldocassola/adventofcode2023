use day4::sum_win_tickets;
use std::{env, fs};
fn main() -> Result<(), String> {
    let args = env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        return Err(String::from("invalid arguments"));
    }

    println!(
        "{}",
        sum_win_tickets(&fs::read_to_string(&args[1]).expect("should exist"))
    );
    Ok(())
}
