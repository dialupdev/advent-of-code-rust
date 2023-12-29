// https://adventofcode.com/2020/day/3

fn check_slope(input: &str, x: usize, y: usize) -> usize {
    input
        .lines()
        .step_by(y)
        .enumerate()
        .filter(|(i, line)| line.chars().nth((i * x) % line.len()).unwrap() == '#')
        .count()
}

fn part1(input: &str) -> usize {
    check_slope(input, 3, 1)
}

fn part2(input: &str) -> usize {
    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|(x, y)| check_slope(input, *x, *y))
        .product()
}

fn main() {
    let input = include_str!("../../input/03.txt").trim_end();

    println!("part 1: {}", part1(input));
    println!("part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("../../input/03.txt");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT.trim_end()), 203);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT.trim_end()), 3316272960);
    }
}
