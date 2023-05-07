// https://adventofcode.com/2016/day/5

use crypto::{digest::Digest, md5::Md5};
use hex;

fn part1(input: &str) -> String {
    let mut password = String::with_capacity(8);

    let mut hash = Md5::new();
    hash.input_str(input);

    // This is for storing our computed digest later
    let mut digest = vec![0u8; 16];
    let mut i = 0;

    loop {
        let mut hash = hash; // Md5 implements Copy
        hash.input_str(&i.to_string());
        hash.result(&mut digest);

        let hex_digest = hex::encode(&digest);

        // Our hash needs to start with five zeros.
        // The first two full bytes must be 0, and the next half byte must be < 0x10.
        // if digest.iter().take(2).all(|b| *b == 0) && (*digest.iter().nth(2).unwrap() < 0x10) {
        if &hex_digest[0..5] == "00000" {
            println!("i: {}, char {}", i, hex_digest.chars().nth(5).unwrap());
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
    hash.input_str(input);

    // This is for storing our computed digest later
    let mut digest = vec![0u8; 16];
    let mut i = 0;

    loop {
        let mut hash = hash; // Md5 implements Copy
        hash.input_str(&i.to_string());
        hash.result(&mut digest);

        let hex_digest = hex::encode(&digest);

        // Our hash needs to start with five zeros.
        // The first two full bytes must be 0, and the next half byte must be < 0x10.
        // if digest.iter().take(2).all(|b| *b == 0) && (*digest.iter().nth(2).unwrap() < 0x10) {
        // println!("d: {}", &hex_digest[0..4]);
        if &hex_digest[0..5] == "00000" {
            // let position = hex_digest.chars().nth(5).unwrap().to_digit(10);
            if let Some(position) = hex_digest.chars().nth(5).unwrap().to_digit(10) {
                // TODO: Can we cast position instead?
                let position = position as usize;
                if (0..=7).contains(&position) && password.chars().nth(position).unwrap() == '-' {
                    // let position = position.unwrap();
                    let char = hex_digest.chars().nth(6).unwrap();
                    println!(
                        "good hash!! i: {}, position: {}, char: {}",
                        i, position, char
                    );
                    password.replace_range(position..=position, &char.to_string());
                    println!("password: {}", password);
                } else {
                    println!("bad position!! i: {}, position: {}", i, position);
                }
            } else {
                println!(
                    "not a num!! i: {}, position: {}",
                    i,
                    hex_digest.chars().nth(5).unwrap()
                );
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
