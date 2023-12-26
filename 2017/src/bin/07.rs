// https://adventofcode.com/2017/day/7

use std::collections::HashMap;

fn parse_input(input: &str) -> HashMap<&str, &str> {
    let mut mappings: HashMap<&str, &str> = HashMap::new();

    input
        .lines()
        .filter_map(|line| {
            let mut parts = line.split(" -> ");
            let parent = parts.next().unwrap().split(" ").next().unwrap();

            // Filter out discs without children
            match parts.next() {
                Some(children) => Some((parent, children)),
                None => None,
            }
        })
        .for_each(|(parent, children)| {
            children.split(", ").for_each(|child| {
                mappings.insert(child, parent);
            });
        });

    return mappings;
}

fn find_root_disc<'a>(mappings: &'a HashMap<&'a str, &'a str>) -> &'a str {
    // Start with a random disc
    let mut disc = mappings.keys().next().unwrap();

    // Keep going up the tree until we find the root
    while let Some(parent) = mappings.get(disc) {
        disc = parent;
    }

    disc
}

fn part1(input: &str) -> String {
    let mappings = parse_input(input);

    find_root_disc(&mappings).to_owned()
}

fn main() {
    let input = include_str!("../../input/07.txt").trim_end();

    println!("part 1: {}", part1(input));
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("../../input/07.txt");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT.trim_end()), "qibuqqg");
    }
}
