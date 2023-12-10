use std::env;
use std::fs;
use day2::sum_games_power;

fn main() -> Result<(), String> {
    let args = env::args().collect::<Vec<_>>();

    if args.len() != 2 {
        return Err("invalid arguments".to_owned());
    }

    let filename = args[1].to_owned();
    let input = fs::read_to_string(filename).unwrap();
    println!("{}", sum_games_power(&input));
    Ok(())
}