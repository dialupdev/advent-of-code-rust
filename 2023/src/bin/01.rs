// https://adventofcode.com/2023/day/1

use either::Either;
use regex::Regex;
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

fn find_digit_or_word(chars: Either<Chars, Rev<Chars>>, re: &Regex) -> u32 {
    let direction_forward = chars.is_left();
    let mut buffer = String::new();

    chars
        .into_iter()
        .find_map(|char| {
            if char.is_numeric() {
                Some(char.to_digit(10).unwrap())
            } else {
                if direction_forward {
                    buffer.push(char);
                } else {
                    buffer.insert(0, char);
                }

                let re_match = re.find(&buffer);
                if re_match.is_some() {
                    match re_match.unwrap().as_str() {
                        "one" => Some(1),
                        "two" => Some(2),
                        "three" => Some(3),
                        "four" => Some(4),
                        "five" => Some(5),
                        "six" => Some(6),
                        "seven" => Some(7),
                        "eight" => Some(8),
                        "nine" => Some(9),
                        _ => unreachable!(),
                    }
                } else {
                    None
                }
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

fn part2(input: &str) -> u32 {
    let re = Regex::new(r"(one|two|three|four|five|six|seven|eight|nine)").unwrap();

    input
        .lines()
        .map(|line| {
            let first_digit = find_digit_or_word(Either::Left(line.chars()), &re);
            let last_digit = find_digit_or_word(Either::Right(line.chars().rev()), &re);

            first_digit * 10 + last_digit
        })
        .sum::<u32>()
}

fn main() {
    let input = include_str!("../../input/01.txt").trim_end();

    println!("part 1: {}", part1(input));
    println!("part 2: {}", part2(input));
}
