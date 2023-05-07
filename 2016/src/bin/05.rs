// https://adventofcode.com/2016/day/5

use hex;
use md5::{Digest, Md5};

fn part1(input: &str) -> String {
    let mut password = String::with_capacity(8);
    let mut hash = Md5::new();
    hash.update(input);

    let mut i = 0;

    loop {
        let mut hash = hash.clone();
        hash.update(&i.to_string());
        let digest = hash.finalize();

        // Our hash needs to start with five zeros, so the first
        // two bytes must be 0 and the third byte must be < 0x10
        if digest.iter().take(2).all(|b| *b == 0) && (*digest.iter().nth(2).unwrap() < 0x10) {
            let hex_digest = hex::encode(&digest);
            password.push(hex_digest.chars().nth(5).unwrap());

            if password.len() == 8 {
                break;
            }
        }

        i += 1;
    }

    password
}

fn part2(input: &str) -> String {
    let mut password = String::from("--------");
    let mut hash = Md5::new();
    hash.update(input);

    let mut i = 0;

    loop {
        let mut hash = hash.clone();
        hash.update(&i.to_string());
        let digest = hash.finalize();

        // Our hash needs to start with five zeros, so the first
        // two bytes must be 0 and the third byte must be < 0x10
        if digest.iter().take(2).all(|b| *b == 0) && (*digest.iter().nth(2).unwrap() < 0x10) {
            let hex_digest = hex::encode(&digest);

            if let Some(position) = hex_digest.chars().nth(5).unwrap().to_digit(10) {
                // TODO: Can we cast position instead?
                let position = position as usize;

                if (0..=7).contains(&position) && password.chars().nth(position).unwrap() == '-' {
                    let char = hex_digest.chars().nth(6).unwrap();
                    password.replace_range(position..=position, &char.to_string());
                }
            }

            if password.chars().all(|char| char != '-') {
                break;
            }
        }

        i += 1;
    }

    password
}

fn main() {
    let input = "abbhdwsy";

    println!("part 1: {}", part1(input));
    println!("part 2: {}", part2(input));
}
