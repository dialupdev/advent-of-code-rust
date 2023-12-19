// https://adventofcode.com/2022/day/13

use serde_json::{json, Value};
use std::cmp::Ordering;

fn compare_values(left: &[Value], right: &[Value]) -> Ordering {
    for i in 0..left.len() {
        // If the right array has run out of items, they're sorted incorrectly
        if right.get(i).is_none() {
            return Ordering::Greater;
        }

        if left[i].is_number() && right[i].is_number() {
            let left_i = left[i].as_u64().unwrap();
            let right_i = right[i].as_u64().unwrap();

            // If the numbers are the same, keep going
            if left_i == right_i {
                continue;
            }

            // If the numbers are different, return a result
            return left_i.cmp(&right_i);
        } else {
            // Mixed types
            let left_vec: Vec<Value> = if left[i].is_array() {
                left[i].as_array().unwrap().clone()
            } else {
                vec![left[i].clone()]
            };

            let right_vec: Vec<Value> = if right[i].is_array() {
                right[i].as_array().unwrap().clone()
            } else {
                vec![right[i].clone()]
            };

            let result = compare_values(&left_vec, &right_vec);
            if result != Ordering::Equal {
                return result;
            }
        }
    }

    // If the left array has run out of items, they're sorted correctly
    if left.len() < right.len() {
        Ordering::Less
    } else {
        Ordering::Equal
    }
}

fn part1(input: &str) -> usize {
    input
        .split("\n\n")
        .enumerate()
        .map(|(index, group)| {
            let mut lines = group.lines();

            let left: Value = serde_json::from_str(lines.next().unwrap()).unwrap();
            let right: Value = serde_json::from_str(lines.next().unwrap()).unwrap();

            let result = compare_values(left.as_array().unwrap(), right.as_array().unwrap());

            match result {
                Ordering::Less => index + 1,
                _ => 0,
            }
        })
        .sum()
}

fn part2(input: &str) -> usize {
    let divider_1 = json!([[2]]);
    let divider_2 = json!([[6]]);

    let mut packets = input
        .lines()
        .filter(|&line| line != "")
        .map(|line| serde_json::from_str(line).unwrap())
        .collect::<Vec<Value>>();

    packets.extend_from_slice(&[divider_1, divider_2]);

    packets.sort_by(|a, b| compare_values(a.as_array().unwrap(), b.as_array().unwrap()));

    let divider_1_index = packets
        .iter()
        .position(|packet| {
            let packet = packet.as_array().unwrap();
            packet.len() == 1 && packet[0].as_array().unwrap().len() == 1 && packet[0][0] == 2
        })
        .unwrap();

    let divider_2_index = packets
        .iter()
        .position(|packet| {
            let packet = packet.as_array().unwrap();
            packet.len() == 1 && packet[0].as_array().unwrap().len() == 1 && packet[0][0] == 6
        })
        .unwrap();

    (divider_1_index + 1) * (divider_2_index + 1)
}

fn main() {
    let input = include_str!("../../input/13.txt").trim_end();

    println!("part 1: {}", part1(input));
    println!("part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("../../input/13.txt");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT.trim_end()), 5852);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT.trim_end()), 24190);
    }
}
