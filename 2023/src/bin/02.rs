// https://adventofcode.com/2023/day/2

use regex::Regex;
use std::cmp::max;

fn parse_set<'a>(set: &'a str, set_re: &Regex) -> (u32, &'a str) {
    let set_caps = set_re.captures(set).unwrap();
    let count = set_caps.get(1).unwrap().as_str().parse::<u32>().unwrap();
    let color = set_caps.get(2).unwrap().as_str();

    (count, color)
}

fn part1(input: &str) -> u32 {
    let game_re = Regex::new(r"Game (\d+): (.+)").unwrap();
    let set_re = Regex::new(r"(\d+) (red|green|blue)").unwrap();

    input
        .lines()
        .filter_map(|line| {
            let game_caps = game_re.captures(line).unwrap();
            let game_id = game_caps.get(1).unwrap().as_str().parse::<u32>().unwrap();
            let game_content = game_caps.get(2).unwrap().as_str();

            if game_content.split("; ").all(|turn| {
                turn.split(", ").all(|set| {
                    let (count, color) = parse_set(set, &set_re);

                    match color {
                        "red" => count <= 12,
                        "green" => count <= 13,
                        "blue" => count <= 14,
                        _ => false,
                    }
                })
            }) {
                Some(game_id)
            } else {
                None
            }
        })
        .sum::<u32>()
}

fn part2(input: &str) -> u32 {
    let game_re = Regex::new(r"Game \d+: (.+)").unwrap();
    let set_re = Regex::new(r"(\d+) (red|green|blue)").unwrap();

    input
        .lines()
        .map(|line| {
            let game_caps = game_re.captures(line).unwrap();
            let game_content = game_caps.get(1).unwrap().as_str();

            let mut red_count = 0;
            let mut green_count = 0;
            let mut blue_count = 0;

            game_content.split("; ").for_each(|turn| {
                turn.split(", ").for_each(|set| {
                    let (count, color) = parse_set(set, &set_re);

                    match color {
                        "red" => red_count = max(red_count, count),
                        "green" => green_count = max(green_count, count),
                        "blue" => blue_count = max(blue_count, count),
                        _ => unreachable!(),
                    }
                })
            });

            red_count * green_count * blue_count
        })
        .sum::<u32>()
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
        assert_eq!(super::part1(INPUT.trim_end()), 2105);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT.trim_end()), 72422);
    }
}
