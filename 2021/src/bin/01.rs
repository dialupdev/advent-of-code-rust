// https://adventofcode.com/2021/day/1

use itertools::Itertools;

fn sum_tuple(t: &(&str, &str, &str)) -> u16 {
    t.0.parse::<u16>().unwrap() + t.1.parse::<u16>().unwrap() + t.2.parse::<u16>().unwrap()
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .tuple_windows()
        .filter(|(a, b)| b.parse::<u16>().unwrap() > a.parse::<u16>().unwrap())
        .count()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .tuple_windows::<(_, _, _)>()
        .tuple_windows()
        .filter(|(a, b)| sum_tuple(b) > sum_tuple(a))
        .count()
}

fn main() {
    let input = include_str!("../../input/01.txt").trim_end();

    println!("part 1: {}", part1(input));
    println!("part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("../../input/01.txt");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT.trim_end()), 1529);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT.trim_end()), 1567);
    }
}
