// https://adventofcode.com/2015/day/5

use fancy_regex::Regex;

fn part1(input: &str) -> usize {
    let vowels = Regex::new(r"[aeiou].*[aeiou].*[aeiou]").unwrap();
    let twice = Regex::new(r"(\w)\1").unwrap();
    let bad = Regex::new(r"ab|cd|pq|xy").unwrap();

    input
        .lines()
        .filter(|line| {
            vowels.is_match(line).unwrap()
                && twice.is_match(line).unwrap()
                && !bad.is_match(line).unwrap()
        })
        .count()
}

fn part2(input: &str) -> usize {
    let double = Regex::new(r"(\w\w).*\1").unwrap();
    let single = Regex::new(r"(\w).\1").unwrap();

    input
        .lines()
        .filter(|line| double.is_match(line).unwrap() && single.is_match(line).unwrap())
        .count()
}

fn main() {
    let input = include_str!("../../input/05.txt").trim_end();

    println!("part 1: {}", part1(input));
    println!("part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("../../input/05.txt");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT.trim_end()), 238);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT.trim_end()), 69);
    }
}
