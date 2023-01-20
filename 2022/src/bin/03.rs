// https://adventofcode.com/2022/day/3

fn char_priority(char: char) -> u16 {
    let char_code = char as u16;

    if char_code >= 97 {
        char_code - 96
    } else {
        char_code - 38
    }
}

fn part1(input: &str) -> u16 {
    input
        .lines()
        .map(|line| {
            let half1 = &line[..(line.len() / 2)];
            let half2 = &line[(line.len() / 2)..];
            let char = half1.chars().find(|&char| half2.contains(char));

            char_priority(char.unwrap())
        })
        .sum()
}

fn part2(input: &str) -> u16 {
    input
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|group| {
            let char = group[0]
                .chars()
                .find(|&char| group[1].contains(char) && group[2].contains(char));

            char_priority(char.unwrap())
        })
        .sum()
}

fn main() {
    let input = include_str!("../../input/03.txt").trim_end();

    println!("part 1: {}", part1(input));
    println!("part 2: {}", part2(input));
}
