// https://adventofcode.com/2015/day/1

fn part1(input: &str) -> i16 {
    input.chars().fold(
        0,
        |floor, char| if char == '(' { floor + 1 } else { floor - 1 },
    )
}

fn part2(input: &str) -> usize {
    input
        .chars()
        // scan is actually filter_scan which is why its iterator yields an option
        .scan(0, |floor, char| {
            if char == '(' {
                *floor += 1;
            } else {
                *floor -= 1;
            }
            Some(*floor)
        })
        .position(|floor| floor < 0)
        .unwrap()
        + 1
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
        assert_eq!(super::part1(INPUT.trim_end()), 280);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT.trim_end()), 1797);
    }
}
