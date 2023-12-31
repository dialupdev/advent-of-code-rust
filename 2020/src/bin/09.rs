// https://adventofcode.com/2020/day/9

use std::collections::VecDeque;

fn part1(input: &str) -> i64 {
    let mut previous = input
        .lines()
        .take(25)
        .map(|line| line.parse::<i64>().unwrap())
        .collect::<VecDeque<i64>>();

    input
        .lines()
        .skip(25)
        .find_map(|line| {
            let number = line.parse::<i64>().unwrap();

            if previous
                .iter()
                .find(|&&a| {
                    let difference: i64 = number as i64 - a;

                    if previous.contains(&difference) {
                        true
                    } else {
                        false
                    }
                })
                .is_some()
            {
                previous.pop_front();
                previous.push_back(number);

                None
            } else {
                Some(number)
            }
        })
        .unwrap()
}

fn main() {
    let input = include_str!("../../input/09.txt").trim_end();

    println!("part 1: {}", part1(input));
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("../../input/09.txt");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT.trim_end()), 2089807806);
    }
}
