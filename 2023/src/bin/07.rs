// https://adventofcode.com/2023/day/7

use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashMap;

fn hand_type(hand: &str) -> u8 {
    let mut frequency_map: HashMap<char, u8> = HashMap::new();

    hand.chars().for_each(|char| {
        *frequency_map.entry(char).or_insert(0) += 1;
    });

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

fn label_value(label: char) -> u8 {
    match label {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        _ => label.to_digit(10).unwrap() as u8,
    }
}

fn compare_hand_values(a: &str, b: &str) -> Ordering {
    for (i, char) in a.chars().enumerate() {
        let a_value = label_value(char);
        let b_value = label_value(b.chars().nth(i).unwrap());
        let comparison = a_value.cmp(&b_value);

        if comparison != Ordering::Equal {
            return comparison;
        }
    }

    Ordering::Equal
}

fn part1(input: &str) -> u32 {
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
        let a_hand_type = hand_type(a.0);
        let b_hand_type = hand_type(b.0);
        let type_comparison = a_hand_type.cmp(&b_hand_type);

        match type_comparison {
            // If types are equal, compare individual card values
            Ordering::Equal => compare_hand_values(a.0, b.0),
            _ => type_comparison,
        }
    });

    turns
        .iter()
        .enumerate()
        .map(|(i, turn)| turn.1 * (i + 1) as u32)
        .sum()
}

fn main() {
    let input = include_str!("../../input/07.txt").trim_end();

    println!("part 1: {}", part1(input));
}
