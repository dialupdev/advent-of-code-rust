// https://adventofcode.com/2022/day/6

fn find_unique_set(input: &str, length: u16) -> u16 {
    let mut recent_unique_chars: Vec<char> = vec![];

    for (i, char) in input.chars().enumerate() {
        let char_position = recent_unique_chars.iter().position(|c| c == &char);

        match char_position {
            Some(position) => {
                // If this char is already in the list, remove
                // the chars up to the match and add the new char
                recent_unique_chars.drain(..position + 1);
                recent_unique_chars.push(char);
            }
            None => {
                // If not, just add the new char to the list
                recent_unique_chars.push(char);

                if recent_unique_chars.len() as u16 == length {
                    return i as u16 + 1;
                }
            }
        }
    }

    0
}

fn part1(input: &str) -> u16 {
    find_unique_set(input, 4)
}

fn part2(input: &str) -> u16 {
    find_unique_set(input, 14)
}

fn main() {
    let input = include_str!("../../input/06-full.txt").trim_end();

    println!("part 1: {}", part1(input));
    println!("part 2: {}", part2(input));
}
