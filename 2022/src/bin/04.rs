// https://adventofcode.com/2022/day/4

use itertools::Itertools;
use regex::Regex;
use std::fs::read_to_string;

fn parse_line(re: &Regex, line: &str) -> (u16, u16, u16, u16) {
    // https://docs.rs/regex/latest/regex/#example-avoid-compiling-the-same-regex-in-a-loop
    re.captures(line)
        .unwrap()
        .iter()
        .skip(1)
        .map(|m| m.unwrap().as_str().parse::<u16>().unwrap())
        .collect_tuple()
        .unwrap()
}

fn part1(input: &str) -> u16 {
    let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();

    input
        .lines()
        .filter(|line| {
            let (start1, end1, start2, end2) = parse_line(&re, line);
            (start1 <= start2 && end1 >= end2) || (start2 <= start1 && end2 >= end1)
        })
        .collect::<Vec<&str>>()
        .len() as u16
}

fn part2(input: &str) -> u16 {
    let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();

    input
        .lines()
        .filter(|line| {
            let (start1, end1, start2, end2) = parse_line(&re, line);
            (start1 <= start2 && end1 >= start2) || (start2 <= start1 && end2 >= start1)
        })
        .collect::<Vec<&str>>()
        .len() as u16
}

fn main() {
    let file_path = "./2022/input/04-full.txt";

    let input = read_to_string(file_path).expect("Can't read file");
    let input = input.trim_end();

    println!("part 1: {}", part1(input));
    println!("part 2: {}", part2(input));
}
