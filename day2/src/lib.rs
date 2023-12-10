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

fn parse_ball_count(count_color: &str) -> (i32, Color) {
    let num_color = count_color
        .trim_start()
        .trim_end()
        .split(' ')
        .collect::<Vec<_>>();

    (
        num_color[0].parse::<i32>().unwrap(),
        num_color[1].parse::<Color>().unwrap(),
    )
}

fn is_possible(game_str: &str, config: &Vec<i32>) -> bool {
    for trial in game_str.split(';') {
        for count_ball in trial.split(',') {
            let (count, color) = parse_ball_count(count_ball);

            if config[color as usize] < count {
                return false;
            }
        }
    }

    true
}

pub fn sum_games_power(games: &str) -> i64 {
    games.lines()
        .map(|l| game_power(l))
        .sum()
}

fn game_power(game: &str) -> i64 {
    game_min_config(game).iter()
        .fold(1 as i64, |acc, val | acc * val.to_owned() as i64 )

}

fn game_min_config(game_str: &str) -> Vec<i32> {
    game_body_min_config(game_str.split(':').last().unwrap())
}

fn game_body_min_config(game_body_str: &str) -> Vec<i32> {
    let mut red: i32 = 0;
    let mut green: i32 = 0;
    let mut blue: i32 = 0;

    for trial in game_body_str.split(';') {
        for ball_count in trial.split(',') {
            let (count, color) = parse_ball_count(ball_count);
            match color {
                Color::Red => {
                    if count > red {
                        red = count;
                    }
                }
                Color::Green => {
                    if count > green {
                        green = count;
                    }
                }
                Color::Blue => {
                    if count > blue {
                        blue = count;
                    }
                }
            }
        }
    }

    vec![red, green, blue]
}

pub fn possible_game_sum(input: &str, config: &Vec<i32>) -> i32 {
    let mut possible = Vec::<i32>::new();

    for game in input.lines() {
        let head_body = game.split(":").collect::<Vec<_>>();

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

    #[test]
    fn test_part2() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let expected = vec![
            vec![4, 2, 6],
            vec![1, 3, 4],
            vec![20, 13, 6],
            vec![14, 3, 15],
            vec![6, 3, 2],
        ];
        let expected_power = vec![
            48, 12, 1560, 630, 36
        ];

        for (pos, ln) in input.lines().enumerate() {
            assert_eq!(expected[pos], game_min_config(ln));
            assert_eq!(expected_power[pos], game_power(ln));
        }

        assert_eq!(2286, sum_games_power(input));
    }
}
