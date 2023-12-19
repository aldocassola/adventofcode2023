use std::{cmp::min, collections::HashSet};

pub fn total_tickets(cards: &str) -> i32 {
    get_generated_cards(cards).iter().sum()
}

fn get_generated_cards(cards: &str) -> Vec<i32> {
    let mut card_count: Vec<i32> = vec![1; cards.lines().count()];
    let max_idx = card_count.len() - 1;

    for (card_idx, ln) in cards.lines().enumerate() {
        let matches = ticket_wins(ln);
        let last_idx = min(card_idx + matches as usize, max_idx);

        for idx in card_idx + 1..=last_idx {
            card_count[idx] += card_count[card_idx];
        }
    }

    card_count
}

pub fn sum_win_tickets(input: &str) -> i32 {
    input.lines().map(|ln| ticket_points(ln)).sum()
}

fn ticket_wins(card: &str) -> i32 {
    let body = get_card_body(card);
    let win_nums = get_win_nums(body);
    let this_nums = get_card_nums(body);
    let win_set = build_win_set(win_nums);

    ticket_matches(this_nums, win_set)
}

fn ticket_points(card: &str) -> i32 {
    let wins = ticket_wins(card);
    let log_matches = wins - 1;

    (2 as f64).powi(log_matches) as i32
}

fn get_card_body(card: &str) -> &str {
    card.split(':').last().expect("body should exist")
}

fn get_win_nums(card_body: &str) -> &str {
    card_body.split('|').next().expect("winnings should exist")
}

fn get_card_nums(card_body: &str) -> &str {
    card_body
        .split("|")
        .last()
        .expect("card numbers should exist")
}

fn ticket_matches(nums: &str, win_set: HashSet<i32>) -> i32 {
    nums.split(' ')
        .filter(|n_str| match n_str.trim().parse::<i32>().ok() {
            Some(v) => win_set.contains(&v),
            _ => false,
        })
        .count() as i32
}

fn build_win_set(win_nums: &str) -> HashSet<i32> {
    win_nums
        .split(' ')
        .filter_map(|n_str| n_str.trim().parse::<i32>().ok())
        .collect::<HashSet<i32>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";
    #[test]
    fn test_sum_winnings() {
        let ticket_wins = vec![8, 2, 2, 1, 0, 0];
        let input_wins = INPUT.lines().map(|l| ticket_points(l)).collect::<Vec<_>>();
        assert_eq!(input_wins, ticket_wins);
        let result = sum_win_tickets(INPUT);
        assert_eq!(result, 13);
    }

    #[test]
    fn test_total_tickets() {
        let expected_wins = vec![4, 2, 2, 1, 0, 0];
        let expected_totals_per_card = vec![1, 2, 4, 8, 14, 1];
        let expected_total = 30;
        assert_eq!(
            expected_wins,
            INPUT.lines().map(|ln| ticket_wins(ln)).collect::<Vec<_>>()
        );
        assert_eq!(expected_totals_per_card, get_generated_cards(&INPUT));
        assert_eq!(expected_total, total_tickets(&INPUT))
    }
}
