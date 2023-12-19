// https://adventofcode.com/2023/day/6

use std::iter::zip;

fn count_wins(time: u64, record_distance: u64) -> u64 {
    let mut win_count = 0;

    for x in 1..time {
        let distance = x * (time - x);

        if distance > record_distance {
            win_count += 1;
        }
    }

    win_count
}

fn part1(input: &str) -> u64 {
    let numbers = input
        .lines()
        .map(|line| {
            line.split(":")
                .nth(1)
                .unwrap()
                .split_whitespace()
                .map(|number| number.parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
        })
        .collect::<Vec<Vec<u64>>>();

    let [times, record_distances] = &numbers[..] else {
        unreachable!("input is not valid")
    };

    zip(times, record_distances)
        .map(|(time, record_distance)| count_wins(*time, *record_distance))
        .product()
}

fn part2(input: &str) -> u64 {
    let numbers = input
        .lines()
        .map(|line| {
            line.split(":")
                .nth(1)
                .unwrap()
                .split_whitespace()
                .collect::<String>()
                .parse::<u64>()
                .unwrap()
        })
        .collect::<Vec<u64>>();

    let [time, record_distance] = &numbers[..] else {
        unreachable!("input is not valid")
    };

    count_wins(*time, *record_distance)
}

fn main() {
    let input = include_str!("../../input/06.txt").trim_end();

    println!("part 1: {}", part1(input));
    println!("part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("../../input/06.txt");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT.trim_end()), 588588);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT.trim_end()), 34655848);
    }
}
