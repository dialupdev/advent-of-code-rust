// https://adventofcode.com/2022/day/2

use std::fs::read_to_string;

// Input mapping: A/X -> 0, B/Y -> 1, C/Z -> 2
fn parse_line(line: &str) -> (u8, u8) {
    let bytes = line.as_bytes();
    (bytes.first().unwrap() - 65, bytes.last().unwrap() - 88)
}

// Outcome mapping: lose -> 0, draw -> 1, win -> 2
fn get_outcome(theirs: u8, ours: u8) -> u8 {
    (3 - (2 + theirs - ours) % 3) % 3
}

fn get_ours(theirs: u8, outcome: u8) -> u8 {
    match outcome {
        0 => (theirs + 2) % 3,
        1 => theirs,
        2 => (theirs + 1) % 3,
        _ => unreachable!(),
    }
}

fn part1(input: &str) -> u16 {
    input
        .lines()
        .map(|line| {
            let (theirs, ours) = parse_line(line);
            let outcome = get_outcome(theirs, ours);
            (outcome * 3 + ours + 1) as u16
        })
        .sum()
}

fn part2(input: &str) -> u16 {
    input
        .lines()
        .map(|line| {
            let (theirs, outcome) = parse_line(line);
            let ours = get_ours(theirs, outcome);
            (outcome * 3 + ours + 1) as u16
        })
        .sum()
}

fn main() {
    let file_path = "./2022/input/02-full.txt";

    let input = read_to_string(file_path).expect("Can't read file");
    let input = input.trim_end();

    println!("part 1: {}", part1(input));
    println!("part 2: {}", part2(input));
}
