// https://adventofcode.com/2016/day/1

use std::collections::HashSet;
use Direction::*;

enum Direction {
    North,
    East,
    South,
    West,
}

fn change_direction(direction: &mut Direction, hand: char) {
    match (&direction, hand) {
        (North, 'L') => *direction = West,
        (North, 'R') => *direction = East,
        (East, 'L') => *direction = North,
        (East, 'R') => *direction = South,
        (South, 'L') => *direction = East,
        (South, 'R') => *direction = West,
        (West, 'L') => *direction = South,
        (West, 'R') => *direction = North,
        _ => unreachable!(),
    }
}

fn move_in_direction(location: &mut (i16, i16), direction: &Direction) {
    match direction {
        North => location.1 -= 1,
        East => location.0 += 1,
        South => location.1 += 1,
        West => location.0 -= 1,
    }
}

fn part1(input: &str) -> i16 {
    let mut direction = Direction::North;
    let mut location = (0, 0); // (x, y)

    input.split(", ").for_each(|instruction| {
        let hand = instruction.chars().nth(0).unwrap();
        change_direction(&mut direction, hand);

        let distance = instruction[1..].parse::<i16>().unwrap();
        for _ in 0..distance {
            move_in_direction(&mut location, &direction);
        }
    });

    location.0.abs() + location.1.abs()
}

fn part2(input: &str) -> i16 {
    let mut direction = Direction::North;
    let mut location = (0, 0); // (x, y)
    let mut visited = HashSet::new();
    visited.insert(location);

    let visited_twice = input.split(", ").find_map(|instruction| {
        let hand = instruction.chars().nth(0).unwrap();
        change_direction(&mut direction, hand);

        let distance = instruction[1..].parse::<i16>().unwrap();
        for _ in 0..distance {
            move_in_direction(&mut location, &direction);

            if !visited.insert(location) {
                return Some(location.0.abs() + location.1.abs());
            }
        }

        None
    });

    visited_twice.unwrap()
}

fn main() {
    let input = include_str!("../../input/01.txt").trim_end();

    println!("part 1: {}", part1(input));
    println!("part 2: {}", part2(input));
}
