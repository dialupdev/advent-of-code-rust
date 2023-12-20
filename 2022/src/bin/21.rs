// https://adventofcode.com/2022/day/21

use std::collections::HashMap;

fn create_monkeys(input: &str) -> HashMap<&str, &str> {
    let mut monkeys: HashMap<&str, &str> = HashMap::new();

    input.lines().for_each(|line| {
        let mut parts = line.split(": ");
        let key = parts.next().unwrap();
        let value = parts.next().unwrap();

        monkeys.insert(key, value);
    });

    return monkeys;
}

fn operate(operator: char, x: u64, y: u64) -> u64 {
    match operator {
        '+' => x + y,
        '-' => x - y,
        '*' => x * y,
        '/' => x / y,
        _ => unreachable!(),
    }
}

fn get_value(monkeys: &HashMap<&str, &str>, key: &str) -> u64 {
    let value = monkeys.get(key).unwrap();

    match value.parse::<u16>() {
        // The value is a number
        Ok(number) => number as u64,
        // The value is an equation
        Err(_) => {
            let mut parts = value.split(" ");
            let key1 = parts.next().unwrap();
            let operator = parts.next().unwrap().chars().next().unwrap();
            let key2 = parts.next().unwrap();

            let value1 = get_value(monkeys, key1);
            let value2 = get_value(monkeys, key2);

            operate(operator, value1, value2)
        }
    }
}

fn part1(input: &str) -> u64 {
    let monkeys = create_monkeys(input);

    get_value(&monkeys, "root")
}

fn main() {
    let input = include_str!("../../input/21.txt").trim_end();

    println!("part 1: {}", part1(input));
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("../../input/21.txt");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT.trim_end()), 121868120894282);
    }
}
