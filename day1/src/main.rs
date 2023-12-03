use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("not enough arguments");
        process::exit(1);
    }

    let filename = args[1].clone();
    let contents = fs::read_to_string(&filename).expect("reading input");
    let mut total = 0;

    for line in contents.lines() {
        let first_digit = line
            .chars()
            .find(|a| a.is_digit(10))
            .and_then(|a| a.to_digit(10))
            .unwrap();
        let second_digit = line
            .chars()
            .rfind(|a| a.is_digit(10))
            .and_then(|a| a.to_digit(10))
            .unwrap();
        total += first_digit * 10 + second_digit;
    }

    println!("{}", total);
}
