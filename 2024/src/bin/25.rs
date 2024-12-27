// https://adventofcode.com/2024/day/25

fn key_fits(lock: &Vec<u8>, key: &Vec<u8>) -> bool {
    lock.iter()
        .enumerate()
        .all(|(index, height)| key[index] + *height <= 5)
}

fn part1(input: &str) -> u16 {
    let mut locks = vec![];
    let mut keys = vec![];

    input.split("\n\n").for_each(|matrix| {
        let marker = matrix.lines().next().unwrap();
        let count = matrix.lines().count();

        let mut heights: Vec<u8> = vec![0; count - 2];

        matrix.lines().skip(1).take(count - 2).for_each(|line| {
            line.chars().enumerate().for_each(|(j, char)| {
                if char == '#' {
                    heights[j] += 1;
                }
            })
        });

        if marker == "#####" {
            locks.push(heights)
        } else {
            keys.push(heights)
        }
    });

    locks
        .iter()
        .map(|lock| keys.iter().filter(|key| key_fits(&lock, &key)).count() as u16)
        .sum()
}

fn main() {
    let input = include_str!("../../input/25.txt").trim_end();

    println!("part 1: {}", part1(input));
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("../../input/25.txt");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT.trim_end()), 3307);
    }
}
