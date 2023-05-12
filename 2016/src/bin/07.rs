// https://adventofcode.com/2016/day/7

use itertools::Itertools;

fn has_abba(sequence: &str) -> bool {
    for (a, b, c, d) in sequence.chars().tuple_windows() {
        if a != b && a == d && b == c {
            return true;
        }
    }

    false
}

fn find_abas_and_get_babs(sequence: &str) -> Vec<String> {
    sequence
        .chars()
        .tuple_windows()
        .filter_map(|(a, b, c)| {
            if a != b && a == c {
                Some(format!("{}{}{}", b, a, b))
            } else {
                None
            }
        })
        .collect()
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let sequences: Vec<_> = line.split(&['[', ']']).collect();
            let mut outers = sequences.iter().step_by(2);
            let mut inners = sequences.iter().skip(1).step_by(2);

            inners.all(|sequence| !has_abba(sequence)) && outers.any(|sequence| has_abba(sequence))
        })
        .count()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let sequences: Vec<_> = line.split(&['[', ']']).collect();
            let outers = sequences.iter().step_by(2);
            let mut inners = sequences.iter().skip(1).step_by(2);

            let babs: Vec<String> = outers
                .flat_map(|sequence| find_abas_and_get_babs(sequence))
                .collect();

            if babs.len() > 0 {
                inners.any(|sequence| babs.iter().any(|bab| sequence.contains(bab)))
            } else {
                false
            }
        })
        .count()
}

fn main() {
    let input = include_str!("../../input/07.txt").trim_end();

    println!("part 1: {}", part1(input));
    println!("part 2: {}", part2(input));
}
