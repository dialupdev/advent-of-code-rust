// https://adventofcode.com/2015/day/4

use crypto::{digest::Digest, md5::Md5};

fn lowest_suffix_with_zeros(input: &str, zeros: usize) -> u32 {
    let mut prefix_hash = Md5::new();
    prefix_hash.input_str(input);

    // This is for storing our computed digest later
    let mut digest = vec![0u8; 16];

    (1..)
        .find(|n| {
            let mut hash = prefix_hash; // Md5 implements Copy
            hash.input_str(&n.to_string());
            hash.result(&mut digest);

            // All full bytes must be 0, and any half bytes must be < 0x10
            digest.iter().take(zeros / 2).all(|b| *b == 0)
                && (zeros % 2 == 0 || *digest.iter().nth(zeros / 2).unwrap() < 0x10)
        })
        .unwrap()
}

fn part1(input: &str) -> u32 {
    lowest_suffix_with_zeros(input, 5)
}

fn part2(input: &str) -> u32 {
    lowest_suffix_with_zeros(input, 6)
}

fn main() {
    let input = "ckczppom";

    println!("part 1: {}", part1(input));
    println!("part 2: {}", part2(input));
}
