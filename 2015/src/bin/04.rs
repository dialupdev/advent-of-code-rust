// https://adventofcode.com/2015/day/4

use md5;

fn part1(input: &str) -> u32 {
    (1..)
        .find(|n| {
            let digest = md5::compute(format!("{}{}", input, n));
            // The first two bytes should be 0, and the third should be < 0x10
            digest.iter().take(2).all(|b| *b == 0) && *digest.iter().nth(2).unwrap() < 0x10
        })
        .unwrap()
}

fn part2(input: &str) -> u32 {
    (1..)
        .find(|n| {
            let digest = md5::compute(format!("{}{}", input, n));
            // The first three bytes should be 0
            digest.iter().take(3).all(|b| *b == 0)
        })
        .unwrap()
}

fn main() {
    let input = "ckczppom";

    println!("part 1: {}", part1(input));
    println!("part 2: {}", part2(input));
}
