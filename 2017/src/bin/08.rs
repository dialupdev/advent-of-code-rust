// https://adventofcode.com/2017/day/8

use itertools::Itertools;
use regex::Regex;
use std::{cmp, collections::HashMap};

fn populate_registers(input: &str) -> (HashMap<String, i16>, i16) {
    let re = Regex::new(r"(\w+) (inc|dec) (-?\d+) if (\w+) (.+) (-?\d+)").unwrap();
    let mut registers: HashMap<String, i16> = HashMap::new();
    let mut max_value = 0;

    for line in input.lines() {
        let (x, operation, operation_amount, y, condition, condition_amount) = re
            .captures(line)
            .unwrap()
            .iter()
            .skip(1)
            .map(|m| m.unwrap().as_str())
            .collect_tuple()
            .unwrap();

        let y_value = registers.entry(y.to_owned()).or_default();
        let condition_amount = condition_amount.parse::<i16>().unwrap();

        let condition_true = match condition {
            ">" => *y_value > condition_amount,
            "<" => *y_value < condition_amount,
            ">=" => *y_value >= condition_amount,
            "<=" => *y_value <= condition_amount,
            "==" => *y_value == condition_amount,
            "!=" => *y_value != condition_amount,
            _ => unreachable!(),
        };

        let x_value = registers.entry(x.to_owned()).or_default();
        let operation_amount = operation_amount.parse::<i16>().unwrap();

        if condition_true {
            match operation {
                "inc" => *x_value += operation_amount,
                "dec" => *x_value -= operation_amount,
                _ => unreachable!(),
            }
        }

        max_value = cmp::max(max_value, *x_value);
    }

    (registers, max_value)
}

fn part1(input: &str) -> i16 {
    let (registers, _) = populate_registers(input);
    *registers.values().max().unwrap()
}

fn part2(input: &str) -> i16 {
    let (_, max_value) = populate_registers(input);
    max_value
}

fn main() {
    let input = include_str!("../../input/08.txt").trim_end();

    println!("part 1: {}", part1(input));
    println!("part 2: {}", part2(input));
}
