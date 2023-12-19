// https://adventofcode.com/2023/day/4

use regex::Regex;
use std::collections::HashMap;

fn count_wins(line: &str, re: &Regex) -> u32 {
    let caps = re.captures(line).unwrap();

    let winning_numbers = caps
        .get(1)
        .unwrap()
        .as_str()
        .split_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let my_numbers = caps
        .get(2)
        .unwrap()
        .as_str()
        .split_whitespace()
        .map(|s| s.parse::<u32>().unwrap());

    my_numbers
        .filter(|my_number| winning_numbers.contains(&my_number))
        .count() as u32
}

fn part1(input: &str) -> u16 {
    let re = Regex::new(r": (.+) \| (.+)").unwrap();

    input
        .lines()
        .map(|line| {
            let win_count = count_wins(line, &re);

            match win_count {
                0 => 0,
                _ => 2_u16.pow(win_count - 1),
            }
        })
        .sum::<u16>()
}

fn part2(input: &str) -> u32 {
    let re = Regex::new(r": (.+) \| (.+)").unwrap();

    let mut card_id = 0;
    let mut card_count = 0;
    let mut card_multipliers: HashMap<usize, u32> = HashMap::new();

    input.lines().for_each(|line| {
        card_id += 1;
        let win_count = count_wins(line, &re);
        let multipler = *card_multipliers.entry(card_id as usize).or_insert(1);

        for i in card_id + 1..=(card_id + win_count) {
            *card_multipliers.entry(i as usize).or_insert(1) += multipler;
        }

        card_count += multipler;
    });

    card_count
}

fn main() {
    let input = include_str!("../../input/04.txt").trim_end();

    println!("part 1: {}", part1(input));
    println!("part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("../../input/04.txt");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT.trim_end()), 24733);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT.trim_end()), 5422730);
    }
}
