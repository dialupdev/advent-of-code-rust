// https://adventofcode.com/2022/day/5

use itertools::Itertools;
use regex::Regex;

fn populate_stacks(input: &str) -> (Vec<Vec<char>>, usize) {
    let divider_index = input.lines().position(|line| line == "").unwrap();

    let column_indexes = input
        .lines()
        .nth(divider_index - 1)
        .unwrap()
        .chars()
        .enumerate()
        .filter_map(|(i, char)| if char.is_numeric() { Some(i) } else { None })
        .collect::<Vec<usize>>();

    let mut stacks: Vec<Vec<char>> = vec![vec![]; column_indexes.len()];

    let stacks_input = input.lines().take(divider_index - 1).collect::<Vec<&str>>();
    stacks_input.iter().rev().for_each(|line| {
        for (i, column_index) in column_indexes.iter().enumerate() {
            let char = line.chars().nth(*column_index).unwrap();
            if char.is_alphabetic() {
                stacks[i].push(char);
            }
        }
    });

    (stacks, divider_index)
}

fn parse_line(re: &Regex, line: &str) -> (usize, usize, usize) {
    re.captures(line)
        .unwrap()
        .iter()
        .skip(1)
        .map(|m| m.unwrap().as_str().parse::<usize>().unwrap())
        .collect_tuple()
        .unwrap()
}

fn part1(input: &str) -> String {
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let (mut stacks, divider_index) = populate_stacks(input);

    input.lines().skip(divider_index + 1).for_each(|line| {
        let (count, from, to) = parse_line(&re, line);

        for _ in 0..count {
            let char = stacks[from - 1].pop().unwrap();
            stacks[to - 1].push(char);
        }
    });

    stacks.iter().map(|stack| stack.last().unwrap()).collect()
}

fn part2(input: &str) -> String {
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let (mut stacks, divider_index) = populate_stacks(input);

    input.lines().skip(divider_index + 1).for_each(|line| {
        let (count, from, to) = parse_line(&re, line);

        let split = stacks[from - 1].len() - count;
        let chars = stacks[from - 1].split_off(split);
        stacks[to - 1].extend(chars)
    });

    stacks.iter().map(|stack| stack.last().unwrap()).collect()
}

fn main() {
    let input = include_str!("../../input/05.txt").trim_end();

    println!("part 1: {}", part1(input));
    println!("part 2: {}", part2(input));
}
