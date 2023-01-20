// https://adventofcode.com/2018/day/2

use std::collections::HashMap;
use std::collections::HashSet;

fn variations_without_one_char(line: &str) -> Vec<(String, usize)> {
    let chars = line.chars();

    chars
        .enumerate()
        .map(|(i, _)| {
            let removed = line.chars().take(i).chain(line.chars().skip(i + 1)).collect::<String>();
            // We have to store the position of the removed char too in order to
            // differentiate between variations in strings with repeated adjacent chars
            (removed, i)
        })
        .collect()
}

fn part1(input: &str) -> u16 {
    let mut doubles = 0;
    let mut triples = 0;

    input.lines().for_each(|line| {
        let mut char_counts: HashMap<char, u16> = HashMap::new();

        line.chars().for_each(|char| {
            *char_counts.entry(char).or_default() += 1;
        });

        if char_counts.values().any(|&f| f == 2) {
            doubles += 1;
        }

        if char_counts.values().any(|&f| f == 3) {
            triples += 1;
        }
    });

    doubles * triples
}

// This variations algorithm is fast but takes a lot of space
fn part2(input: &str) -> String {
    let mut known_variations = HashSet::new();

    input
        .lines()
        .find_map(|line| {
            variations_without_one_char(line)
                .into_iter()
                .find(|variation| !known_variations.insert(variation.clone()))
        })
        .unwrap()
        .0
}

fn main() {
    let input = include_str!("../../input/02.txt").trim_end();

    println!("part 1: {}", part1(input));
    println!("part 2: {}", part2(input));
}
