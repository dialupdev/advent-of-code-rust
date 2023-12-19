// https://adventofcode.com/2017/day/6

use std::collections::{HashMap, HashSet};

fn parse_input(input: &str) -> Vec<u16> {
    input
        .split('\t')
        .map(|bank| bank.parse::<u16>().unwrap())
        .collect()
}

fn find_max_index(banks: &[u16]) -> usize {
    // Get the max index with tie break going to the earlier number
    // rather than the later, which is the default in Rust
    banks.len()
        - banks
            .iter()
            .rev()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.cmp(b))
            .unwrap()
            .0
        - 1
}

fn redistribute_blocks(banks: &mut Vec<u16>, i: usize) {
    let block_count = banks[i];
    banks[i] = 0;
    let mut j = i;

    for _ in 0..block_count {
        if j == banks.len() - 1 {
            j = 0;
        } else {
            j += 1;
        }

        banks[j] += 1;
    }
}

fn part1(input: &str) -> usize {
    let mut known_configurations: HashSet<Vec<u16>> = HashSet::new();
    let mut banks = parse_input(input);
    let mut i;
    let mut count = 0;

    loop {
        if !known_configurations.insert(banks.clone()) {
            break;
        }

        i = find_max_index(&banks);
        redistribute_blocks(&mut banks, i);

        count += 1;
    }

    count
}

fn part2(input: &str) -> usize {
    let mut known_configurations: HashMap<Vec<u16>, usize> = HashMap::new();
    let mut banks = parse_input(input);
    let mut i;
    let mut count = 0;

    loop {
        if let Some(previous_count) = known_configurations.get(&banks) {
            return count - previous_count;
        }

        known_configurations.insert(banks.clone(), count);

        i = find_max_index(&banks);
        redistribute_blocks(&mut banks, i);

        count += 1;
    }
}

fn main() {
    let input = include_str!("../../input/06.txt").trim_end();

    println!("part 1: {}", part1(input));
    println!("part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("../../input/06.txt");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT.trim_end()), 11137);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT.trim_end()), 1037);
    }
}
