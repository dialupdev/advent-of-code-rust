// https://adventofcode.com/2021/day/6

use std::collections::HashMap;

fn run_simulation_for_n_days(input: &str, days: u16) -> u64 {
    let mut fish_by_cycle: HashMap<u8, u64> = HashMap::from([
        (0, 0),
        (1, 0),
        (2, 0),
        (3, 0),
        (4, 0),
        (5, 0),
        (6, 0),
        (7, 0),
        (8, 0),
    ]);

    input
        .split(",")
        .for_each(|n| *fish_by_cycle.get_mut(&n.parse().unwrap()).unwrap() += 1);

    for _ in 0..days {
        // We'll start with 0 since there is no cycle 9
        let mut previous_cycle_count = 0;

        // Modify each cycle group to become the previous cycle group
        for cycle in (0..=8).rev() {
            let this_cycle_count_ref = fish_by_cycle.get_mut(&cycle).unwrap();
            let this_cycle_count = *this_cycle_count_ref;

            *this_cycle_count_ref = previous_cycle_count;
            previous_cycle_count = this_cycle_count;
        }

        // Each fish that spawned a new fish will become cycle 6
        *fish_by_cycle.get_mut(&6).unwrap() += previous_cycle_count;

        // Spawn new fish
        *fish_by_cycle.get_mut(&8).unwrap() = previous_cycle_count;
    }

    fish_by_cycle.values().sum()
}

fn part1(input: &str) -> u64 {
    run_simulation_for_n_days(input, 80)
}

fn part2(input: &str) -> u64 {
    run_simulation_for_n_days(input, 256)
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
        assert_eq!(super::part1(INPUT.trim_end()), 360268);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT.trim_end()), 1632146183902);
    }
}
