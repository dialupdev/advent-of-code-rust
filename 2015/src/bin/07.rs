// https://adventofcode.com/2015/day/7

use std::collections::HashMap;

fn parse_input(input: &str) -> HashMap<&str, &str> {
    let mut wires: HashMap<&str, &str> = HashMap::new();

    input.lines().for_each(|line| {
        let mut split = line.split(" -> ");
        let formula = split.next().unwrap();
        let key = split.next().unwrap();

        wires.insert(key, formula);
    });

    wires
}

fn parse_or_get_value<'a>(
    wires: &HashMap<&'a str, &'a str>,
    values: &mut HashMap<&'a str, u16>,
    key: &'a str,
) -> u16 {
    match values.get(key) {
        Some(value) => *value,
        None => match key.parse::<u16>() {
            Ok(value) => value,
            Err(_) => {
                let value = get_value(wires, values, key);
                values.insert(key, value);

                value
            }
        },
    }
}

fn get_value<'a>(
    wires: &HashMap<&'a str, &'a str>,
    values: &mut HashMap<&'a str, u16>,
    key: &'a str,
) -> u16 {
    let formula = *wires.get(key).unwrap();
    let mut split = formula.split_whitespace();

    match split.clone().count() {
        1 => {
            // Assignment
            parse_or_get_value(wires, values, formula)
        }
        2 => {
            // Bitwise NOT
            let a = get_value(wires, values, split.nth(1).unwrap());
            !a
        }
        3 => {
            let a = split.next().unwrap();
            let operator = split.next().unwrap();
            let b = split.next().unwrap();

            match operator {
                "AND" | "OR" => {
                    let a = parse_or_get_value(wires, values, a);
                    let b = get_value(wires, values, b);

                    match operator {
                        // Bitwise AND/OR
                        "AND" => a & b,
                        "OR" => a | b,
                        _ => unreachable!(),
                    }
                }
                "LSHIFT" | "RSHIFT" => {
                    let a = get_value(wires, values, a);
                    let b = b.parse::<u16>().unwrap();

                    match operator {
                        // Bit shift
                        "LSHIFT" => a << b,
                        "RSHIFT" => a >> b,
                        _ => unreachable!(),
                    }
                }
                _ => unreachable!(),
            }
        }
        _ => unreachable!(),
    }
}

fn part1(input: &str) -> u16 {
    let wires = parse_input(input);
    let mut values: HashMap<&str, u16> = HashMap::new();

    get_value(&wires, &mut values, "a")
}

fn part2(input: &str) -> u16 {
    let mut wires = parse_input(input);
    let mut values: HashMap<&str, u16> = HashMap::new();

    wires.entry("b").and_modify(|entry| *entry = "16076");

    get_value(&wires, &mut values, "a")
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
        assert_eq!(super::part1(INPUT.trim_end()), 16076);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT.trim_end()), 2797);
    }
}
