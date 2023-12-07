use std::iter;

pub fn extract_digit_total(input: &str) -> u32 {
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

pub fn extract_digit_word_total(input: &str) -> u32 {
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
                                idx += 1;
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
        let first_digit = digits.first().expect("should be a valid digit");
        let second_digit = digits.last().expect("should be a valid digit");
        let num_str = format!("{first_digit}{second_digit}");
        let nn = num_str.parse::<u32>().expect("should be valid 2 digit number");
        total += nn
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
        let vals = vec![
            12, 38, 15, 77,
        ];
        assert_eq!(extract_digit_total(input), 142);
        for (pos, ln) in input.lines().enumerate() {
            assert_eq!(vals[pos], extract_digit_total(&ln));
        }
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
        let vals = vec![
            29, 83, 13, 24, 42, 14, 76,
        ];
        assert_eq!(extract_digit_word_total(input), 281);
        for (pos, ln) in input.lines().enumerate() {
            assert_eq!(vals[pos], extract_digit_word_total(&ln));
        }
        assert_eq!(extract_digit_word_total("twone"), 21);
    }
}
