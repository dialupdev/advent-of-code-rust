// https://adventofcode.com/2018/day/3

use regex::Regex;
use std::collections::HashSet;

#[derive(Debug)]
struct Rect {
    x: u16,
    y: u16,
    width: u16,
    height: u16,
}

fn parse_line(re: &Regex, line: &str) -> Rect {
    let caps = re.captures(line).unwrap();

    Rect {
        x: caps[1].parse::<u16>().unwrap(),
        y: caps[2].parse::<u16>().unwrap(),
        width: caps[3].parse::<u16>().unwrap(),
        height: caps[4].parse::<u16>().unwrap(),
    }
}

fn part1(input: &str) -> usize {
    let re = Regex::new(r"(\d+),(\d+): (\d+)x(\d+)").unwrap();
    let mut claims = HashSet::new();
    let mut disputed_claims = HashSet::new();

    input.lines().for_each(|line| {
        let rect = parse_line(&re, line);

        for x in rect.x..rect.x + rect.width {
            for y in rect.y..rect.y + rect.height {
                if !claims.insert((x, y)) {
                    disputed_claims.insert((x, y));
                }
            }
        }
    });

    disputed_claims.len()
}

fn main() {
    let input = include_str!("../../input/03.txt").trim_end();

    println!("part 1: {}", part1(input));
}
