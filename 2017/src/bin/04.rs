// https://adventofcode.com/2017/day/4

use itertools::Itertools;
use std::collections::HashSet;

fn part1(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let words: Vec<&str> = line.split_whitespace().collect();
            let words_len = words.len();
            let unique_words: HashSet<&str> = HashSet::from_iter(words);

            words_len == unique_words.len()
        })
        .count()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let anagrams: Vec<String> = line
                .split_whitespace()
                .map(|word| word.chars().sorted().collect::<String>())
                .collect();
            let anagrams_len = anagrams.len();
            let unique_anagrams: HashSet<String> = HashSet::from_iter(anagrams);

            anagrams_len == unique_anagrams.len()
        })
        .count()
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
        assert_eq!(super::part1(INPUT.trim_end()), 477);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT.trim_end()), 167);
    }
}
