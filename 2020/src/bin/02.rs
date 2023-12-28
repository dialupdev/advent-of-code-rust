// https://adventofcode.com/2020/day/2

use regex::Regex;

fn part1(input: &str) -> usize {
    let re = Regex::new(r"(\d+)-(\d+) (\w): (\w+)").unwrap();

    input
        .lines()
        .filter(|line| {
            let caps = re.captures(line).unwrap();
            let min = caps[1].parse::<usize>().unwrap();
            let max = caps[2].parse::<usize>().unwrap();
            let letter = caps[3].chars().next().unwrap();
            let password = &caps[4];

            let count = password.matches(letter).count();
            count >= min && count <= max
        })
        .count()
}

fn part2(input: &str) -> usize {
    let re = Regex::new(r"(\d+)-(\d+) (\w): (\w+)").unwrap();

    input
        .lines()
        .filter(|line| {
            let caps = re.captures(line).unwrap();
            let index_1 = caps[1].parse::<usize>().unwrap() - 1;
            let index_2 = caps[2].parse::<usize>().unwrap() - 1;
            let letter = caps[3].chars().next().unwrap();
            let password = &caps[4];

            (password.chars().nth(index_1).unwrap() == letter)
                != (password.chars().nth(index_2).unwrap() == letter)
        })
        .count()
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
        assert_eq!(super::part1(INPUT.trim_end()), 445);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT.trim_end()), 491);
    }
}
