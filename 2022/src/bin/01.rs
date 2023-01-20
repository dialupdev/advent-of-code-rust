// https://adventofcode.com/2022/day/1

fn part1(input: &str) -> u32 {
    input
        .split("\n\n")
        .map(|group| group.lines().fold(0, |sum, line| sum + line.parse::<u32>().unwrap()))
        .max()
        .unwrap()
}

fn part2(input: &str) -> u32 {
    let mut groups: Vec<u32> = input
        .split("\n\n")
        .map(|group| group.lines().fold(0, |sum, line| sum + line.parse::<u32>().unwrap()))
        .collect();

    groups.sort_by(|a, b| b.cmp(a));
    groups.iter().take(3).sum()
}

fn main() {
    let input = include_str!("../../input/01.txt").trim_end();

    println!("part 1: {}", part1(input));
    println!("part 2: {}", part2(input));
}
