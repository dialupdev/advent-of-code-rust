// https://adventofcode.com/2023/day/4

use regex::Regex;

fn part1(input: &str) -> u16 {
    let re = Regex::new(r": (.+) \| (.+)").unwrap();

    input
        .lines()
        .map(|line| {
            let caps = re.captures(line).unwrap();

            let winning_numbers = caps
                .get(1)
                .unwrap()
                .as_str()
                .split_whitespace()
                .map(|s| s.parse::<u8>().unwrap())
                .collect::<Vec<u8>>();

            let my_numbers = caps
                .get(2)
                .unwrap()
                .as_str()
                .split_whitespace()
                .map(|s| s.parse::<u8>().unwrap());

            let win_count = my_numbers
                .filter(|my_number| winning_numbers.contains(&my_number))
                .count();

            match win_count {
                0 => 0,
                _ => 2_u16.pow(win_count as u32 - 1),
            }
        })
        .sum::<u16>()
}

fn main() {
    let input = include_str!("../../input/04.txt").trim_end();

    println!("part 1: {}", part1(input));
}
