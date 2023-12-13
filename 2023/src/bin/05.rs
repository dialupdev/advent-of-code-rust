// https://adventofcode.com/2023/day/5

use regex::Regex;
use std::collections::HashMap;

fn find_location(source: &str, number: u64, maps: &HashMap<&str, (&str, Vec<[u64; 3]>)>) -> u64 {
    let map = maps.get(source).unwrap();
    let destination = map.0;
    let rows = &map.1;

    let maybe_row = rows.iter().find(|row| {
        let [_, source_start, length] = row;
        *source_start <= number && (source_start + length) >= number
    });

    let next_number = match maybe_row {
        Some(row) => {
            let [destination_start, source_start, _] = row;
            let shift = *destination_start as i64 - *source_start as i64;
            (number as i64 + shift) as u64
        }
        None => number,
    };

    if destination == "location" {
        next_number
    } else {
        find_location(destination, next_number, maps)
    }
}

fn part1(input: &str) -> u64 {
    let re = Regex::new(r"(\w+)-to-(\w+) map:").unwrap();

    let mut groups = input.split("\n\n");

    let seeds = groups
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|n| n.parse::<u64>().unwrap());

    let mut maps: HashMap<&str, (&str, Vec<[u64; 3]>)> = HashMap::new();

    groups.for_each(|group| {
        let mut lines = group.lines();

        let caps = re.captures(lines.next().unwrap()).unwrap();
        let source = caps.get(1).unwrap().as_str();
        let destination = caps.get(2).unwrap().as_str();

        let map = lines
            .map(|line| {
                let mut split = line.split_whitespace().map(|n| n.parse::<u64>().unwrap());
                [
                    split.next().unwrap(),
                    split.next().unwrap(),
                    split.next().unwrap(),
                ]
            })
            .collect::<Vec<[u64; 3]>>();

        maps.insert(source, (destination, map));
    });

    seeds
        .map(|seed| find_location("seed", seed, &maps))
        .min()
        .unwrap()
}

fn main() {
    let input = include_str!("../../input/05.txt").trim_end();

    println!("part 1: {}", part1(input));
}
