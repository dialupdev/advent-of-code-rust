// https://adventofcode.com/2020/day/9

use std::collections::VecDeque;

fn part1(input: &str) -> i64 {
    let mut previous = input
        .lines()
        .take(25)
        .map(|line| line.parse::<i64>().unwrap())
        .collect::<VecDeque<i64>>();

    input
        .lines()
        .skip(25)
        .find_map(|line| {
            let number = line.parse::<i64>().unwrap();

            if previous
                .iter()
                .find(|&a| {
                    let difference: i64 = number as i64 - *a;

                    if previous.contains(&difference) {
                        true
                    } else {
                        false
                    }
                })
                .is_some()
            {
                previous.pop_front();
                previous.push_back(number);

                None
            } else {
                Some(number)
            }
        })
        .unwrap()
}

fn part2(input: &str) -> i64 {
    let values = input
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let mut start_index = 0;
    let mut end_index = 1;
    let mut candidates = VecDeque::from([values[start_index], values[end_index]]);
    let mut sum = values[start_index] + values[end_index];

    loop {
        if sum == 2089807806 {
            break;
        }

        if sum < 2089807806 {
            end_index += 1;
            candidates.push_back(values[end_index]);
            sum += values[end_index];
        } else {
            sum -= values[start_index];
            candidates.pop_front();
            start_index += 1;
        }
    }

    candidates.iter().min().unwrap() + candidates.iter().max().unwrap()
}

fn main() {
    let input = include_str!("../../input/09.txt").trim_end();

    println!("part 1: {}", part1(input));
    println!("part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("../../input/09.txt");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT.trim_end()), 2089807806);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT.trim_end()), 245848639);
    }
}
