use itertools::Itertools;
use std::collections::BTreeMap;
use std::ops::Add;

pub fn sum_gear_ratios(engine_schematic: &str) -> i64 {
    let gears = get_gears(&build_engine_map(engine_schematic));
    gears.iter().map(|g| g.ratio).fold(0, |acc, gr| acc + gr)
}

fn get_gears(engine_map: &BTreeMap<Position, Value>) -> Vec<Gear> {
    let numbers = get_numbers(engine_map);
    let nums_tree = numbers
        .iter()
        .enumerate()
        .flat_map(|(idx, numv)| numv.iter().map(move |(pos, _)| (*pos, idx)))
        .collect::<BTreeMap<Position, usize>>();
    let stars = engine_map.iter().filter_map(|(p, v)| {
        if matches!(v, Value::Symbol('*')) {
            Some(p)
        } else {
            None
        }
    });

    stars
        .filter_map(|star_pos| {
            let adjs = get_adj_list(star_pos).collect::<Vec<_>>();
            let num_idxs = adjs
                .iter()
                .filter_map(|p| {
                    if let Some(idx) = nums_tree.get(p) {
                        Some(idx)
                    } else {
                        None
                    }
                })
                .unique()
                .collect::<Vec<_>>();

            if num_idxs.len() == 2 {
                let pair = num_idxs
                    .iter()
                    .map(|idx| {
                        numbers[**idx]
                            .iter()
                            .map(|e| e.1)
                            .collect::<String>()
                            .parse::<i32>()
                            .expect("should be a number")
                    })
                    .collect::<Vec<_>>();
                Some(Gear {
                    pos: *star_pos,
                    ratio: pair[0] as i64 * pair[1] as i64,
                })
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
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

    let numbers = get_numbers(&engine_map);
    let mut total: i32 = 0;
    for num in numbers {
        let num_pos = num.iter().map(|(p, _)| p).collect::<Vec<_>>();
        let mut check_pos = num
            .iter()
            .flat_map(|(pos, _)| {
                get_adj_list(pos).filter_map(|adjp| {
                    if num_pos.contains(&&adjp)
                        || adjp.x < 0
                        || (adjp.x as usize) >= max_chars
                        || adjp.y < 0
                        || (adjp.y as usize) >= max_lines
                    {
                        None
                    } else {
                        Some(adjp)
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

fn get_adj_list(pos: &Position) -> impl Iterator<Item = Position> + '_ {
    [
        Position { x: 0, y: 1 },
        Position { x: 0, y: -1 },
        Position { x: 1, y: 0 },
        Position { x: -1, y: 0 },
        Position { x: 1, y: -1 },
        Position { x: -1, y: 1 },
        Position { x: 1, y: 1 },
        Position { x: -1, y: -1 },
    ]
    .iter()
    .map(|adj| *adj + *pos)
}

fn get_numbers(engine_map: &BTreeMap<Position, Value>) -> Vec<Vec<(Position, char)>> {
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

    numbers
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

#[derive(Debug, PartialEq, Eq)]
struct Gear {
    pos: Position,
    ratio: i64,
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

    #[test]
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
            Gear {
                pos: Position { y: 1, x: 3 },
                ratio: 467 * 35,
            },
            Gear {
                pos: Position { y: 8, x: 5 },
                ratio: 755 * 598,
            },
        ];
        let gears = get_gears(&build_engine_map(input));
        assert_eq!(gears, expected_gears);
        assert_eq!(expected, sum_gear_ratios(input));
    }
}
