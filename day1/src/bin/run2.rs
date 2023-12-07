use std::fs;
use std::env;
use std::process;

fn main() {
    let args : Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("not enough arguments");
        process::exit(1);
    }

    let filename = args[1].clone();
    let contents = fs::read_to_string(&filename).expect("reading input");
    let total = day1::extract_digit_word_total(&contents);

    println!("{}", total);
}