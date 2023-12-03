use std::env;
use std::fs;
use std::process;

fn main() {
    let args : Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("not enough arguments");
        process::exit(1);
    }

    let filename = args[1].clone();
    let contents = fs::read_to_string(&filename).expect("reading input");
    let mut total = 0;

    for line in contents.lines() {
        let first_digit = line.find(char::is_numeric).expect("finding first digit");
        let last_digit = line.rfind(char::is_numeric).expect("finding last digit");
        total += first_digit * 10 + last_digit;
    }

    println!("{}", total);
}
