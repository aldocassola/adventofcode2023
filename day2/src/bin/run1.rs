use std::env;
use std::fs;
use std::process;
use day2::possible_game_sum;
use day2::Color;

fn main() {
    let args = env::args().collect::<Vec<_>>();

    if args.len() < 2 {
        eprint!("invalid arguments");
        process::exit(-1);
    }

    let filename = args[1].to_owned();
    let input = fs::read_to_string(filename).expect("reading file");
    let mut config = vec![0,0,0];
    config[Color::Red as usize] = 12;
    config[Color::Green as usize] = 13;
    config[Color::Blue as usize] = 14;

    println!("{}", possible_game_sum(&input, &config))
}