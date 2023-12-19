// https://adventofcode.com/2022/day/1

fn part1(input: &str) -> u32 {
    input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .fold(0, |sum, line| sum + line.parse::<u32>().unwrap())
        })
        .max()
        .unwrap()
}

fn part2(input: &str) -> u32 {
    let mut groups: Vec<u32> = input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .fold(0, |sum, line| sum + line.parse::<u32>().unwrap())
        })
        .collect();

    groups.sort_by(|a, b| b.cmp(a));
    groups.iter().take(3).sum()
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
        assert_eq!(super::part1(INPUT.trim_end()), 71023);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT.trim_end()), 206289);
    }
}
