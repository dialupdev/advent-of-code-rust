// https://adventofcode.com/2017/day/1

fn sum_by_offset(input: &str, offset: usize) -> u32 {
    let bytes = input.as_bytes();
    let len = bytes.len();
    let mut sum = 0u32;

    for i in 0..len {
        let a = bytes[i] as u32 - 48;
        let b = bytes[(i + offset) % len] as u32 - 48;

        if a == b {
            sum += a;
        }
    }

    sum
}

fn part1(input: &str) -> u32 {
    sum_by_offset(input, 1)
}

fn part2(input: &str) -> u32 {
    sum_by_offset(input, input.len() / 2)
}

fn main() {
    let input = include_str!("../../input/01.txt").trim_end();

    println!("part 1: {}", part1(input));
    println!("part 2: {}", part2(input));
}
