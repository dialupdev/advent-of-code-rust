// https://adventofcode.com/2015/day/6

use regex::Regex;
use std::cmp;

#[derive(Debug)]
enum Action {
    TurnOn,
    TurnOff,
    Toggle,
}

struct Operation {
    action: Action,
    x_start: usize,
    y_start: usize,
    x_end: usize,
    y_end: usize,
}

fn parse_line(re: &Regex, line: &str) -> Operation {
    let caps = re.captures(line).unwrap();
    let action = match &caps[1] {
        "turn on" => Action::TurnOn,
        "turn off" => Action::TurnOff,
        "toggle" => Action::Toggle,
        &_ => unreachable!(),
    };

    Operation {
        action,
        x_start: caps[2].parse::<usize>().unwrap(),
        y_start: caps[3].parse::<usize>().unwrap(),
        x_end: caps[4].parse::<usize>().unwrap(),
        y_end: caps[5].parse::<usize>().unwrap(),
    }
}

fn part1(input: &str) -> usize {
    let re = Regex::new(r"^(.*) (\d+),(\d+) through (\d+),(\d+)").unwrap();
    let mut grid = [[false; 1000]; 1000];

    input.lines().for_each(|line| {
        let op = parse_line(&re, line);

        for x in op.x_start..=op.x_end {
            for y in op.y_start..=op.y_end {
                grid[x][y] = match op.action {
                    Action::TurnOn => true,
                    Action::TurnOff => false,
                    Action::Toggle => !grid[x][y],
                };
            }
        }
    });

    grid.iter()
        .map(|column| column.iter().filter(|&light| *light).count())
        .sum()
}

fn part2(input: &str) -> i32 {
    let re = Regex::new(r"^(.*) (\d+),(\d+) through (\d+),(\d+)").unwrap();
    let mut grid = [[0i32; 1000]; 1000];

    input.lines().for_each(|line| {
        let op = parse_line(&re, line);

        for x in op.x_start..=op.x_end {
            for y in op.y_start..=op.y_end {
                grid[x][y] = match op.action {
                    Action::TurnOn => grid[x][y] + 1,
                    Action::TurnOff => cmp::max(grid[x][y] - 1, 0),
                    Action::Toggle => grid[x][y] + 2,
                };
            }
        }
    });

    grid.iter().map(|column| column.iter().sum::<i32>()).sum()
}

fn main() {
    let input = include_str!("../../input/06.txt").trim_end();

    println!("part 1: {}", part1(input));
    println!("part 2: {}", part2(input));
}
