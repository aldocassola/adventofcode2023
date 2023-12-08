use std::str::FromStr;

#[repr(u8)]
pub enum Color {
    Red = 0,
    Green,
    Blue,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ColorParseError;

impl FromStr for Color {
    type Err = ColorParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "red" => Ok(Color::Red),
            "green" => Ok(Color::Green),
            "blue" => Ok(Color::Blue),
            _ => Err(ColorParseError),
        }
    }
}

fn is_possible(game_str: &str, config: &Vec<i32>) -> bool {
    for trial in game_str.split(';') {
        for count_ball in trial.split(',') {
            let color_ball = count_ball.trim_start().trim_end().split(' ').collect::<Vec<_>>();

            if color_ball.len() != 2 {
                eprint!("bad ball count:{:?}", color_ball);
                return false;
            }

            let count = match color_ball[0].parse::<i32>() {
                Ok(c) => c,
                Err(msg) => {
                    eprint!("bad count:{msg}");
                    return false;
                }
            };

            let color = color_ball[1].parse::<Color>().expect("bad color");

            if config[color as usize] < count {
                return false;
            }
        }
    }

    true
}

pub fn possible_game_sum(input: &str, config: &Vec<i32>) -> i32 {
    let mut possible = Vec::<i32>::new();

    for game in input.lines() {
        let head_body = game.split(":").collect::<Vec<_>>();
        if head_body.len() != 2 {
            eprintln!("bad game {game}");
            continue;
        }

        let game_number = head_body[0].split(" ").collect::<Vec<_>>();
        let game_id = game_number[1].parse::<i32>().expect("should be a number");

        if !is_possible(&head_body[1], config) {
            continue;
        }

        possible.push(game_id);
    }

    possible.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let mut config = vec![0, 0, 0];
        config[Color::Red as usize] = 12;
        config[Color::Green as usize] = 13;
        config[Color::Blue as usize] = 14;
        assert_eq!(possible_game_sum(&input, &config), 8);
    }
}
