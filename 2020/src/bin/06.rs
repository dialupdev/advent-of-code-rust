// https://adventofcode.com/2020/day/6

use std::collections::{HashMap, HashSet};

fn part1(input: &str) -> usize {
    let mut answers: HashSet<char> = HashSet::new();

    input
        .split("\n\n")
        .map(|group| {
            group.lines().flat_map(|line| line.chars()).for_each(|c| {
                answers.insert(c);
            });

            let count = answers.len();

            answers.clear();

            count
        })
        .sum()
}

fn part2(input: &str) -> usize {
    let mut answers: HashMap<char, usize> = HashMap::new();

    input
        .split("\n\n")
        .map(|group| {
            group.lines().flat_map(|line| line.chars()).for_each(|c| {
                *answers.entry(c).or_insert(0) += 1;
            });

            let count = answers
                .iter()
                .filter(|&c| *c.1 == group.lines().count())
                .count();

            answers.clear();

            count
        })
        .sum()
}

fn main() {
    let input = include_str!("../../input/06.txt").trim_end();

    println!("part 1: {}", part1(input));
    println!("part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("../../input/06.txt");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT.trim_end()), 6521);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT.trim_end()), 3305);
    }
}
