// https://adventofcode.com/2020/day/1

fn part1(input: &str) -> u32 {
    let entries = input
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    entries
        .iter()
        .find_map(|&a| {
            let difference = 2020 - a;

            if entries.contains(&difference) {
                Some(a * difference)
            } else {
                None
            }
        })
        .unwrap()
}

// https://en.wikipedia.org/wiki/3SUM
fn part2(input: &str) -> u32 {
    let entries = input
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    entries
        .iter()
        .find_map(|&a| {
            entries.iter().find_map(|&b| {
                let difference: i32 = 2020 - a as i32 - b as i32;

                if difference > 0 && entries.contains(&(difference as u32)) {
                    Some(a * b * difference as u32)
                } else {
                    None
                }
            })
        })
        .unwrap()
}

fn main() {
    let input = include_str!("../../input/01.txt").trim_end();

    println!("part 1: {}", part1(input));
    println!("part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("../../input/01.txt");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT.trim_end()), 800139);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT.trim_end()), 59885340);
    }
}
