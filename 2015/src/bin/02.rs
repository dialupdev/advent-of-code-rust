// https://adventofcode.com/2015/day/2

use itertools::Itertools;
use regex::Regex;

fn parse_line(re: &Regex, line: &str) -> (u16, u16, u16) {
    re.captures(line)
        .unwrap()
        .iter()
        .skip(1)
        .map(|n| n.unwrap().as_str().parse::<u16>().unwrap())
        .collect_tuple()
        .unwrap()
}

fn part1(input: &str) -> u32 {
    let re = Regex::new(r"(\d+)x(\d+)x(\d+)").unwrap();

    input
        .lines()
        .map(|line| {
            let (l, w, h) = parse_line(&re, line);
            let areas = [l * w, w * h, h * l];

            areas
                .iter()
                .fold(0, |total, area| total + 2 * (*area as u32))
                + (*areas.iter().min().unwrap() as u32)
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    let re = Regex::new(r"(\d+)x(\d+)x(\d+)").unwrap();

    input
        .lines()
        .map(|line| {
            let (l, w, h) = parse_line(&re, line);
            let perimeters = [l * 2 + w * 2, w * 2 + h * 2, h * 2 + l * 2];

            (*perimeters.iter().min().unwrap() as u32) + (l * w * h) as u32
        })
        .sum()
}

fn main() {
    let input = include_str!("../../input/02.txt").trim_end();

    println!("part 1: {}", part1(input));
    println!("part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("../../input/02.txt");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT.trim_end()), 1588178);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT.trim_end()), 3783758);
    }
}
