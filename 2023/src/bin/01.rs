// https://adventofcode.com/2023/day/1

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut digits = line
                .chars()
                .filter(|char| char.is_numeric())
                .map(|char| char.to_digit(10).unwrap())
                .peekable();

            *digits.peek().unwrap() * 10 + digits.last().unwrap()
        })
        .sum::<u32>()
}

fn main() {
    let input = include_str!("../../input/01.txt").trim_end();

    println!("part 1: {}", part1(input));
    // println!("part 2: {}", part2(input));
}
