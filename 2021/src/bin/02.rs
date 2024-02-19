// https://adventofcode.com/2021/day/2

fn part1(input: &str) -> u32 {
    let mut position = 0;
    let mut depth = 0;

    input.lines().for_each(|line| {
        let mut split = line.split_whitespace();
        let direction = split.next().unwrap();
        let amount = split.next().unwrap().parse::<u32>().unwrap();

        match direction {
            "forward" => position += amount,
            "down" => depth += amount,
            "up" => depth -= amount,
            _ => {}
        };
    });

    position * depth
}

fn part2(input: &str) -> u32 {
    let mut position = 0;
    let mut depth = 0;
    let mut aim = 0;

    input.lines().for_each(|line| {
        let mut split = line.split_whitespace();
        let direction = split.next().unwrap();
        let amount = split.next().unwrap().parse::<u32>().unwrap();

        match direction {
            "forward" => {
                position += amount;
                depth += aim * amount;
            }
            "down" => aim += amount,
            "up" => aim -= amount,
            _ => {}
        };
    });

    position * depth
}

fn main() {
    let input = include_str!("../../input/02.txt").trim_end();

    println!("part 1: {}", part1(input));
    println!("part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("../../input/02.txt");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT.trim_end()), 1727835);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT.trim_end()), 1544000595);
    }
}
