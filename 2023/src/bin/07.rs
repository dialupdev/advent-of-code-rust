// https://adventofcode.com/2023/day/7

use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashMap;

fn hand_type(hand: &str, joker_rule: bool) -> u8 {
    let mut frequency_map: HashMap<char, u8> = HashMap::new();

    hand.chars().for_each(|char| {
        *frequency_map.entry(char).or_insert(0) += 1;
    });

    if joker_rule {
        let most_frequent_label = frequency_map
            .iter()
            .filter(|char| char.0 != &'J')
            .max_by(|(_, a), (_, b)| a.cmp(b));

        let joker_count = *frequency_map.get(&'J').unwrap_or(&0);

        // If there are any jokers, transfer their score to the most frequent label
        if most_frequent_label.is_some() && joker_count > 0 {
            *frequency_map
                .entry(*most_frequent_label.unwrap().0)
                .or_insert(0) += joker_count;
            frequency_map.remove(&'J');
        }
    }

    let frequencies = frequency_map
        .values()
        .map(|f| *f)
        .sorted_by(|a, b| b.cmp(a))
        .collect::<Vec<_>>();

    if frequencies[0] == 5 {
        6 // Five of a kind
    } else if frequencies[0] == 4 {
        5 // Four of a kind
    } else if frequencies[0] == 3 && frequencies[1] == 2 {
        4 // Full house
    } else if frequencies[0] == 3 {
        3 // Three of a kind
    } else if frequencies[0] == 2 && frequencies[1] == 2 {
        2 // Two pair
    } else if frequencies[0] == 2 {
        1 // One pair
    } else {
        0 // High card
    }
}

fn label_value(label: char, joker_rule: bool) -> u8 {
    match label {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => {
            if joker_rule {
                1
            } else {
                11
            }
        }
        'T' => 10,
        _ => label.to_digit(10).unwrap() as u8,
    }
}

fn compare_hand_values(a: &str, b: &str, joker_rule: bool) -> Ordering {
    for (i, char) in a.chars().enumerate() {
        let a_value = label_value(char, joker_rule);
        let b_value = label_value(b.chars().nth(i).unwrap(), joker_rule);
        let comparison = a_value.cmp(&b_value);

        if comparison != Ordering::Equal {
            return comparison;
        }
    }

    Ordering::Equal
}

fn play_game(input: &str, joker_rule: bool) -> u32 {
    let mut turns = input
        .lines()
        .map(|line| {
            let mut parts = line.split(' ');
            let hand = parts.next().unwrap();
            let bid = parts.next().unwrap().parse::<u32>().unwrap();

            (hand, bid)
        })
        .collect::<Vec<_>>();

    turns.sort_by(|a, b| {
        let a_hand_type = hand_type(a.0, joker_rule);
        let b_hand_type = hand_type(b.0, joker_rule);

        let type_comparison = a_hand_type.cmp(&b_hand_type);

        match type_comparison {
            // If types are equal, compare individual card values
            Ordering::Equal => compare_hand_values(a.0, b.0, joker_rule),
            _ => type_comparison,
        }
    });

    turns
        .iter()
        .enumerate()
        .map(|(i, turn)| turn.1 * (i + 1) as u32)
        .sum()
}

fn part1(input: &str) -> u32 {
    play_game(input, false)
}

fn part2(input: &str) -> u32 {
    play_game(input, true)
}

fn main() {
    let input = include_str!("../../input/07.txt").trim_end();

    println!("part 1: {}", part1(input));
    println!("part 2: {}", part2(input));
}
