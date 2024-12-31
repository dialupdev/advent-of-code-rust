// https://adventofcode.com/2024/day/1

use itertools::Itertools;
use std::collections::HashMap;

fn parse(input: &str) -> (Vec<u32>, Vec<u32>) {
    input
        .split_whitespace()
        .map(|n| n.parse::<u32>().unwrap())
        .tuples()
        .unzip()
}

fn part1(input: &str) -> u32 {
    let (mut left, mut right) = parse(input);

    left.sort();
    right.sort();

    left.iter().zip(right).map(|(l, r)| l.abs_diff(r)).sum()
}

fn part2(input: &str) -> u32 {
    let (left, right) = parse(input);

    // This can only be up to 1000 elements (the length of the input)
    let mut frequency_map = HashMap::with_capacity(1_000);

    right.iter().for_each(|r| {
        *frequency_map.entry(r).or_insert(0) += 1;
    });

    left.iter()
        .map(|l| l * frequency_map.get(l).unwrap_or(&0))
        .sum()
}

fn main() {
    let input = include_str!("../../input/01.txt").trim_end();

    println!("part 1: {}", part1(input));
    println!("part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("../../input/01.txt");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT.trim_end()), 1197984);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT.trim_end()), 23387399);
    }
}
