// https://adventofcode.com/2016/day/6

use std::collections::HashMap;

fn frequency_map(input: &str) -> Vec<HashMap<char, u8>> {
    let line_length = input.lines().next().unwrap().len();
    let mut frequencies = vec![HashMap::new(); line_length];

    input.lines().for_each(|line| {
        line.chars().enumerate().for_each(|(i, char)| {
            *frequencies[i].entry(char).or_insert(0) += 1;
        });
    });

    frequencies
}

fn part1(input: &str) -> String {
    frequency_map(input)
        .iter()
        .map(|frequency| frequency.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap().0)
        .collect()
}

fn part2(input: &str) -> String {
    frequency_map(input)
        .iter()
        .map(|frequency| frequency.iter().min_by(|a, b| a.1.cmp(b.1)).unwrap().0)
        .collect()
}

fn main() {
    let input = include_str!("../../input/06.txt").trim_end();

    println!("part 1: {}", part1(input));
    println!("part 2: {}", part2(input));
}
