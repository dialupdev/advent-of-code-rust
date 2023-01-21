// https://adventofcode.com/2015/day/3

use std::collections::HashSet;

// location: (x, y)
fn move_direction(location: &mut (i16, i16), direction: char) {
    match direction {
        '^' => location.1 -= 1,
        '>' => location.0 += 1,
        'v' => location.1 += 1,
        '<' => location.0 -= 1,
        _ => unreachable!(),
    }
}

fn part1(input: &str) -> usize {
    let mut location = (0, 0);
    let mut houses = HashSet::new();
    houses.insert(location);

    input.chars().for_each(|direction| {
        move_direction(&mut location, direction);
        houses.insert(location);
    });

    houses.len()
}

fn part2(input: &str) -> usize {
    let mut santa_location = (0, 0);
    let mut robo_location = (0, 0);
    let mut houses = HashSet::new();
    houses.insert(santa_location);

    input.chars().enumerate().for_each(|(i, direction)| {
        if i % 2 == 0 {
            move_direction(&mut santa_location, direction);
            houses.insert(santa_location);
        } else {
            move_direction(&mut robo_location, direction);
            houses.insert(robo_location);
        }
    });

    houses.len()
}

fn main() {
    let input = include_str!("../../input/03.txt").trim_end();

    println!("part 1: {}", part1(input));
    println!("part 2: {}", part2(input));
}
