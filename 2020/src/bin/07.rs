// https://adventofcode.com/2020/day/7

use regex::Regex;
use std::collections::HashMap;

type StyleMap<'a> = HashMap<&'a str, Vec<(&'a str, u16)>>;

fn create_style_map(input: &str) -> StyleMap {
    let re = Regex::new(r"^(\d+) ([\w\s]+) bags?").unwrap();

    let mut style_map = HashMap::new();

    input.lines().for_each(|line| {
        let mut parts = line.split(" bags contain ");
        let style = parts.next().unwrap();
        let child_styles = parts
            .next()
            .unwrap()
            .split(", ")
            .filter_map(|child| {
                let caps = re.captures(child);

                match caps {
                    Some(caps) => {
                        let style = caps.get(2).unwrap().as_str();
                        let count = caps[1].parse().unwrap();

                        Some((style, count))
                    }
                    None => None,
                }
            })
            .collect();

        style_map.insert(style, child_styles);
    });

    style_map
}

fn contains_shiny_gold(style: &str, depth: u8, style_map: &StyleMap) -> bool {
    if style == "shiny gold" && depth > 0 {
        return true;
    }

    let children = style_map.get(style);

    match children {
        Some(children) => children
            .iter()
            .any(|child| contains_shiny_gold(child.0, depth + 1, style_map)),
        None => false,
    }
}

fn count_children(style: &str, style_map: &StyleMap) -> u16 {
    let children = style_map.get(style);

    match children {
        Some(children) => children
            .iter()
            .map(|child| child.1 + child.1 * count_children(child.0, style_map))
            .sum(),
        None => 0,
    }
}

fn part1(input: &str) -> usize {
    let style_map = create_style_map(input);

    style_map
        .iter()
        .filter(|style| contains_shiny_gold(style.0, 0, &style_map))
        .count()
}

fn part2(input: &str) -> u16 {
    let style_map = create_style_map(input);

    count_children("shiny gold", &style_map)
}

fn main() {
    let input = include_str!("../../input/07.txt").trim_end();

    println!("part 1: {}", part1(input));
    println!("part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("../../input/07.txt");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT.trim_end()), 372);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT.trim_end()), 8015);
    }
}
