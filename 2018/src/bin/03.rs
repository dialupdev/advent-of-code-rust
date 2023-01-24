// https://adventofcode.com/2018/day/3

use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

fn parse_line(re: &Regex, line: &str) -> (u16, u16, u16, u16, u16) {
    re.find_iter(line)
        .map(|m| m.as_str().parse::<u16>().unwrap())
        .collect_tuple()
        .unwrap()
}

fn part1(input: &str) -> usize {
    let re = Regex::new(r"(\d+)").unwrap();
    let mut claims: HashMap<(u16, u16), u16> = HashMap::new();

    input.lines().for_each(|line| {
        let (_, left, top, width, height) = parse_line(&re, line);

        for x in left..left + width {
            for y in top..top + height {
                *claims.entry((x, y)).or_default() += 1;
            }
        }
    });

    claims.values().filter(|&v| *v > 1).count()
}

fn main() {
    let input = include_str!("../../input/03.txt").trim_end();

    println!("part 1: {}", part1(input));
}
