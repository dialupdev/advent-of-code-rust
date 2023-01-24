// https://adventofcode.com/2018/day/3

use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet};

fn mark_claims(input: &str) -> HashMap<(u16, u16), HashSet<u16>> {
    let re = Regex::new(r"(\d+)").unwrap();
    let mut claimed: HashMap<(u16, u16), HashSet<u16>> = HashMap::new();

    input.lines().for_each(|line| {
        let (id, left, top, width, height) = re
            .find_iter(line)
            .map(|m| m.as_str().parse::<u16>().unwrap())
            .collect_tuple()
            .unwrap();

        for x in left..left + width {
            for y in top..top + height {
                claimed.entry((x, y)).or_default().insert(id);
            }
        }
    });

    claimed
}

fn part1(input: &str) -> usize {
    let claimed = mark_claims(input);

    claimed.values().filter(|ids| ids.len() > 1).count()
}

fn part2(input: &str) -> u16 {
    let claimed = mark_claims(input);

    let mut all_claims: HashSet<u16> = HashSet::new();
    let mut contested_claims: HashSet<u16> = HashSet::new();

    claimed.values().for_each(|ids| {
        all_claims.extend(ids);

        if ids.len() > 1 {
            contested_claims.extend(ids);
        }
    });

    *(&all_claims - &contested_claims).iter().next().unwrap()
}

fn main() {
    let input = include_str!("../../input/03.txt").trim_end();

    println!("part 1: {}", part1(input));
    println!("part 2: {}", part2(input));
}
