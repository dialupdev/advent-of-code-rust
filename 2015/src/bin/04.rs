// https://adventofcode.com/2015/day/4

use md5::{Digest, Md5};

fn lowest_suffix_with_zeros(input: &str, zeros: usize) -> u32 {
    let mut hash = Md5::new();
    hash.update(input);

    (1..)
        .find(|n| {
            let mut hash = hash.clone();
            hash.update(&n.to_string());
            let digest = hash.finalize();

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

#[cfg(test)]
mod tests {
    const INPUT: &str = "ckczppom";

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 117946);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), 3938038);
    }
}
