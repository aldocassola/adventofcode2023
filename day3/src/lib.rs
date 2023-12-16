use std::collections::BTreeMap;
use std::ops::Add;

use itertools::Itertools;

pub fn sum_gear_ratios(engine_schematic: &str) -> i32 {
    0
}

fn get_gears(engine_schematic: &str) -> Vec<Gear> {
    let engine_map = build_engine_map(engine_schematic);

    vec![]
}

pub fn sum_parts(engine_schematic: &str) -> i32 {
    let max_lines = engine_schematic
        .lines()
        .enumerate()
        .last()
        .expect("input cannot be empty")
        .0
        + 1;
    let max_chars = engine_schematic
        .lines()
        .last()
        .expect("lines cannot be empty")
        .chars()
        .enumerate()
        .last()
        .expect("line cannot be empty")
        .0
        + 1;
    let engine_map = build_engine_map(engine_schematic);

    let mut numbers: Vec<Vec<(Position, char)>> = vec![];
    for (pos, val) in engine_map.iter() {
        if let Value::Digit(digit) = val {
            match numbers.iter().last() {
                Some(v) => {
                    let last_num = v.iter().last();
                    match last_num {
                        Some((last_pos, _val)) => {
                            if last_pos.x + 1 == pos.x {
                                let last = numbers.iter_mut().last().expect("should exist");

                                last.push((*pos, *digit));
                            } else {
                                numbers.push(vec![(*pos, *digit)]);
                            }
                        }
                        None => unimplemented!("shouldn't happen"),
                    }
                }
                None => numbers.push(vec![(*pos, *digit)]),
            }
        }
    }

    let mut total: i32 = 0;
    for num in numbers {
        let adjdelta = [
            Position { x: 0, y: 1 },
            Position { x: 0, y: -1 },
            Position { x: 1, y: 0 },
            Position { x: -1, y: 0 },
            Position { x: 1, y: -1 },
            Position { x: -1, y: 1 },
            Position { x: 1, y: 1 },
            Position { x: -1, y: -1 },
        ];

        let num_pos = num.iter().map(|(p, _)| p).collect::<Vec<_>>();
        let mut check_pos = num
            .iter()
            .flat_map(|(p, _)| {
                adjdelta.iter().filter_map(|adjp| {
                    let pos = *p + *adjp;
                    if num_pos.contains(&&pos)
                        || pos.x < 0
                        || (pos.x as usize) >= max_chars
                        || pos.y < 0
                        || (pos.y as usize) >= max_lines
                    {
                        None
                    } else {
                        Some(pos)
                    }
                })
            })
            .unique();

        if check_pos.any(|pos| matches!(engine_map.get(&pos), Some(Value::Symbol(_)))) {
            let this_num = num
                .iter()
                .map(|(_, ch)| ch)
                .collect::<String>()
                .parse::<i32>()
                .expect("should be a number");
            total += this_num;
        }
    }

    total
}


fn build_engine_map(input: &str) -> BTreeMap<Position, Value> {
    input
        .lines()
        .enumerate()
        .flat_map(|(lnum, line)| {
            line.chars().enumerate().map(move |(cpos, ch)| {
                let pos = Position {
                    x: cpos as i32,
                    y: lnum as i32,
                };
                match ch {
                    '.' => (pos, Value::Empty),
                    _ if ch.is_digit(10) => (pos, Value::Digit(ch)),
                    sym => (pos, Value::Symbol(sym)),
                }
            })
        })
        .collect::<BTreeMap<_, _>>()
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Hash)]
struct Position {
    y: i32,
    x: i32,
}

impl Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        return Self::Output {
            y: self.y + rhs.y,
            x: self.x + rhs.x,
        };
    }
}

#[derive(Debug)]
enum Value {
    Empty,
    Digit(char),
    Symbol(char),
}

#[derive(Debug)]
struct Gear {
    pos: Position,
    ratio: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_parts() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let expected = 4361;
        let result = sum_parts(&input);
        assert_eq!(result, expected);
    }

    fn test_gear_ratio() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let expected = 467835;
        let expected_gears: Vec<Gear> = vec![
            Gear{pos: Position{y: 1, x: 3}, ratio: 467*35},
            Gear{pos: Position{y: 8, x: 5}, ratio: 755*598},
        ];
        let gears = get_gears(input);
        assert_eq!(expected, sum_gear_ratios(input));
    }
}
