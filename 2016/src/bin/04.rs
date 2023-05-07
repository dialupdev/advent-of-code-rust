// https://adventofcode.com/2016/day/4

use regex::Regex;
use std::cmp::Ordering;
use std::collections::HashMap;

fn parse_line<'a>(re: &Regex, line: &'a str) -> (&'a str, u32, &'a str) {
    let caps = re.captures(line).unwrap();

    (
        caps.get(1).unwrap().as_str(),
        caps.get(2).unwrap().as_str().parse::<u32>().unwrap(),
        caps.get(3).unwrap().as_str(),
    )
}

fn compute_checksum(name: &str) -> String {
    let mut frequency: HashMap<char, u8> = HashMap::new();

    name.chars().filter(|char| *char != '-').for_each(|char| {
        *frequency.entry(char).or_insert(0) += 1;
    });

    let mut frequency: Vec<_> = frequency.iter().collect();

    frequency.sort_by(|a, b| {
        // We reverse because higher frequencies should come first
        if a.1 > b.1 {
            Ordering::Greater.reverse()
        } else if a.1 < b.1 {
            Ordering::Less.reverse()
        } else {
            a.0.cmp(b.0)
        }
    });

    frequency.iter().take(5).map(|(char, _)| *char).collect()
}

fn decrypt_name(name: &str, id: u32) -> String {
    name.bytes()
        .map(|byte| {
            if byte == b'-' {
                ' '
            } else {
                let shift = (id % 26) as u8;
                // 97 is the start of the ASCII lowercase range
                char::from(((byte + shift - 97) % 26) + 97)
            }
        })
        .collect::<String>()
}

fn part1(input: &str) -> u32 {
    let re = Regex::new(r"([\w|-]+)-(\d+)\[(\w+)\]").unwrap();

    input
        .lines()
        .filter_map(|line| {
            let (name, id, checksum) = parse_line(&re, line);

            if compute_checksum(name) == checksum {
                Some(id)
            } else {
                None
            }
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    let re = Regex::new(r"([\w|-]+)-(\d+)\[(\w+)\]").unwrap();

    input
        .lines()
        .find_map(|line| {
            let (name, id, _) = parse_line(&re, line);

            if decrypt_name(name, id) == "northpole object storage" {
                Some(id)
            } else {
                None
            }
        })
        .unwrap()
}

fn main() {
    let input = include_str!("../../input/04.txt").trim_end();

    println!("part 1: {}", part1(input));
    println!("part 2: {}", part2(input));
}
