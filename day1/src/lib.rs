use std::iter;

pub fn run1(input: &str) -> u32 {
    let mut total = 0;

    for line in input.lines() {
        let first_digit = line
            .chars()
            .find(|a| a.is_digit(10))
            .and_then(|a| a.to_digit(10))
            .expect("first digit");
        let second_digit = line
            .chars()
            .rfind(|a| a.is_digit(10))
            .and_then(|a| a.to_digit(10))
            .expect("second digit");
        total += first_digit * 10 + second_digit;
    }

    total
}

pub fn run2(input: &str) -> u32 {
    let mut total = 0;

    for ln in input.lines() {
        let mut idx = 0;
        let it = iter::from_fn(move || {
            let names = vec![
                "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
            ];

            while idx != ln.len() {
                let substr = &ln[idx..];
                match substr.chars().next() {
                    Some(ch) if ch.is_digit(10) => {
                        idx += 1;
                        return Some(ch)
                    }
                    Some(_) => {
                        for (pos, w) in names.iter().enumerate() {
                            if substr.starts_with(w) {
                                idx += w.len();
                                return pos.to_string().chars().next();
                            }
                        }
                    }
                    None => return None,
                }
                idx += 1;
            }
            None
        });

        let digits = it.collect::<Vec<_>>();
        let num_str = format!("{}{}", digits.first().unwrap(), digits.last().unwrap());
        total += num_str.parse::<u32>().unwrap();
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run1() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!(run1(input), 142);
    }

    #[test]
    fn test_run2() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!(run2(input), 281);
    }
}
