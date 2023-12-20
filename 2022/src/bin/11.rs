// https://adventofcode.com/2022/day/11

use regex::Regex;

#[derive(Debug)]
enum Operand {
    Number(u32),
    Old,
}

#[derive(Debug)]
struct Monkey {
    operation: (char, Operand),
    test: u32,
    if_true: usize,
    if_false: usize,
    inspection_count: u32,
}

impl Monkey {
    fn new(operation: (char, Operand), test: u32, if_true: usize, if_false: usize) -> Monkey {
        Monkey {
            operation,
            test,
            if_true,
            if_false,
            inspection_count: 0,
        }
    }
}

fn parse_input(input: &str) -> (Vec<Monkey>, Vec<Vec<u32>>) {
    let re = Regex::new(r"old (\+|\*) (.+)").unwrap();

    let mut monkeys: Vec<Monkey> = vec![];
    let mut bags: Vec<Vec<u32>> = vec![];

    input.split("\n\n").for_each(|group| {
        let lines = group.lines().collect::<Vec<&str>>();

        let items = lines[1]
            .split(": ")
            .nth(1)
            .unwrap()
            .split(", ")
            .map(|num| num.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        bags.push(items);

        let caps = re.captures(lines[2]).unwrap();
        let operand = match &caps[2] {
            "old" => Operand::Old,
            num => Operand::Number(num.parse().unwrap()),
        };
        let operation = (caps[1].chars().nth(0).unwrap(), operand);

        let test = lines[3].split_whitespace().last().unwrap().parse().unwrap();
        let if_true = lines[4].split_whitespace().last().unwrap().parse().unwrap();
        let if_false = lines[5].split_whitespace().last().unwrap().parse().unwrap();

        monkeys.push(Monkey::new(operation, test, if_true, if_false));
    });

    (monkeys, bags)
}

fn do_round(monkeys: &mut Vec<Monkey>, bags: &mut Vec<Vec<u32>>) {
    // During each round, give each monkey a turn
    monkeys.iter_mut().enumerate().for_each(|(i, monkey)| {
        let bag = bags[i].clone();
        bags[i].clear();

        bag.iter().for_each(|item| {
            // Inspect item
            let mut new_item = match monkey.operation {
                ('+', Operand::Number(num)) => item + num,
                ('+', Operand::Old) => item + item,
                ('*', Operand::Number(num)) => item * num,
                ('*', Operand::Old) => item * item,
                _ => unreachable!(),
            };

            // Post-inspection worry reduction
            new_item /= 3;

            // Decide which monkey to throw the item to
            if new_item % monkey.test == 0 {
                bags[monkey.if_true].push(new_item);
            } else {
                bags[monkey.if_false].push(new_item);
            };

            // Increment inspection count
            monkey.inspection_count += 1;
        });
    });
}

fn part1(input: &str) -> u32 {
    let (mut monkeys, mut bags) = parse_input(input);

    for _ in 0..20 {
        do_round(&mut monkeys, &mut bags);
    }

    let mut inspection_counts = monkeys
        .iter()
        .map(|monkey| monkey.inspection_count)
        .collect::<Vec<_>>();

    inspection_counts.sort_by(|a, b| b.cmp(a));

    inspection_counts[0] * inspection_counts[1]
}

fn main() {
    let input = include_str!("../../input/11.txt").trim_end();

    println!("part 1: {}", part1(input));
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("../../input/11.txt");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT.trim_end()), 151312);
    }
}
