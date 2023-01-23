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
