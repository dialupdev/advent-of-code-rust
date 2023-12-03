// https://adventofcode.com/2023/day/1

use either::Either;
use std::{iter::Rev, str::Chars};

fn find_digit(chars: Either<Chars, Rev<Chars>>) -> u32 {
    chars
        .into_iter()
        .find_map(|char| {
            if char.is_numeric() {
                Some(char.to_digit(10).unwrap())
            } else {
                None
            }
        })
        .unwrap()
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let first_digit = find_digit(Either::Left(line.chars()));
            let last_digit = find_digit(Either::Right(line.chars().rev()));

            first_digit * 10 + last_digit
        })
        .sum::<u32>()
}

fn main() {
    let input = include_str!("../../input/01.txt").trim_end();

    println!("part 1: {}", part1(input));
    // println!("part 2: {}", part2(input));
}
