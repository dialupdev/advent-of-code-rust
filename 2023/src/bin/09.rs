// https://adventofcode.com/2023/day/9

use itertools::Itertools;

fn build_sequences(input: &str) -> Vec<Vec<Vec<i32>>> {
    input
        .lines()
        .map(|line| {
            let mut sequences: Vec<Vec<i32>> = vec![];

            // First add the input sequence
            sequences.push(
                line.split_whitespace()
                    .map(|n| n.parse::<i32>().unwrap())
                    .collect::<Vec<_>>(),
            );

            let mut last_sequence = sequences.last().unwrap();
            let mut next_sequence: Vec<i32>;

            // Then generate the following sequences
            while !last_sequence.iter().all(|&n| n == 0) {
                next_sequence = last_sequence
                    .iter()
                    .tuple_windows()
                    .map(|(a, b)| b - a)
                    .collect::<Vec<_>>();

                sequences.push(next_sequence);
                last_sequence = sequences.last().unwrap();
            }

            sequences
        })
        .collect()
}

fn part1(input: &str) -> i32 {
    build_sequences(input)
        .iter()
        .map(|sequences| {
            let mut i = sequences.len();
            let mut last_number = 0;

            while i > 0 {
                i -= 1;
                last_number = *sequences[i].last().unwrap() + last_number;
            }

            last_number
        })
        .sum()
}

fn part2(input: &str) -> i32 {
    build_sequences(input)
        .iter()
        .map(|sequences| {
            let mut i = sequences.len();
            let mut first_number = 0;

            while i > 0 {
                i -= 1;
                first_number = *sequences[i].first().unwrap() - first_number;
            }

            first_number
        })
        .sum()
}

fn main() {
    let input = include_str!("../../input/09.txt").trim_end();

    println!("part 1: {}", part1(input));
    println!("part 2: {}", part2(input));
}
