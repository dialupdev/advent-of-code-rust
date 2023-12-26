// https://adventofcode.com/2015/day/10

use std::mem;

fn look_and_say(sequence: &mut Vec<char>) {
    let char_count = sequence.len();

    let mut last_char_count = 0;
    let mut last_char = *sequence.get(0).unwrap();
    let mut next_sequence: Vec<char> = vec![];

    for (i, &char) in sequence.iter().enumerate() {
        if char == last_char {
            last_char_count += 1;
        } else {
            next_sequence.push(char::from_digit(last_char_count, 10).unwrap());
            next_sequence.push(last_char);

            last_char_count = 1;
            last_char = char;
        }

        if i == char_count - 1 {
            next_sequence.push(char::from_digit(last_char_count, 10).unwrap());
            next_sequence.push(last_char);
        }
    }

    mem::swap(sequence, &mut next_sequence);
}

fn part1(input: &str) -> usize {
    let mut sequence = input.chars().collect::<Vec<char>>();

    for _ in 0..40 {
        look_and_say(&mut sequence);
    }

    sequence.len()
}

fn part2(input: &str) -> usize {
    let mut sequence = input.chars().collect::<Vec<char>>();

    for _ in 0..50 {
        look_and_say(&mut sequence);
    }

    sequence.len()
}

fn main() {
    let input = "1113122113";

    println!("part 1: {}", part1(input));
    println!("part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "1113122113";

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 360154);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), 5103798);
    }
}
