// https://adventofcode.com/2023/day/8

use regex::Regex;
use std::collections::HashMap;

fn part1(input: &str) -> u32 {
    let re = Regex::new(r"(\w{3}) = \((\w{3}), (\w{3})\)").unwrap();

    let mut groups = input.split("\n\n");

    let moves = groups.next().unwrap().chars().collect::<Vec<char>>();
    let mut nodes: HashMap<&str, (&str, &str)> = HashMap::new();

    groups.next().unwrap().lines().for_each(|line| {
        let caps = re.captures(line).unwrap();
        let label = caps.get(1).unwrap().as_str();
        let left = caps.get(2).unwrap().as_str();
        let right = caps.get(3).unwrap().as_str();

        nodes.insert(label, (left, right));
    });

    let mut label = "AAA";
    let mut move_index = 0;
    let mut move_count = 0;
    let mut node;

    while label != "ZZZ" {
        move_count += 1;
        node = nodes.get(label).unwrap();

        match moves[move_index] {
            'L' => {
                label = node.0;
            }
            'R' => {
                label = node.1;
            }
            _ => unreachable!(),
        }

        move_index = (move_index + 1) % moves.len();
    }

    move_count
}

fn main() {
    let input = include_str!("../../input/08.txt").trim_end();

    println!("part 1: {}", part1(input));
    // println!("part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("../../input/08.txt");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT.trim_end()), 14429);
    }
}
