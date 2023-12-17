// https://adventofcode.com/2023/day/16

use std::collections::HashSet;
use Direction::*;

type Location = (i8, i8); // (x, y)

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn maybe_change_direction_and_spawn(
    cave: &Vec<Vec<char>>,
    energy_matrix: &mut Vec<Vec<bool>>,
    known_paths: &mut HashSet<(Location, Direction)>,
    location: Location,
    direction: &mut Direction,
    symbol: char,
) {
    match (&direction, symbol) {
        // Up
        (Up, '|') => *direction = Up,
        (Up, '/') => *direction = Right,
        (Up, '-') => {
            spawn_beam(cave, energy_matrix, known_paths, location, Left);
            *direction = Right;
        }
        (Up, '\\') => *direction = Left,

        // Right
        (Right, '|') => {
            spawn_beam(cave, energy_matrix, known_paths, location, Up);
            *direction = Down;
        }
        (Right, '/') => *direction = Up,
        (Right, '-') => *direction = Right,
        (Right, '\\') => *direction = Down,

        // Down
        (Down, '|') => *direction = Down,
        (Down, '/') => *direction = Left,
        (Down, '-') => {
            spawn_beam(cave, energy_matrix, known_paths, location, Right);
            *direction = Left;
        }
        (Down, '\\') => *direction = Right,

        // Left
        (Left, '|') => {
            spawn_beam(cave, energy_matrix, known_paths, location, Down);
            *direction = Up;
        }
        (Left, '/') => *direction = Down,
        (Left, '-') => *direction = Left,
        (Left, '\\') => *direction = Up,

        _ => unreachable!(),
    }
}

fn move_in_direction(location: &mut Location, direction: &Direction) {
    match direction {
        Up => location.1 -= 1,
        Right => location.0 += 1,
        Down => location.1 += 1,
        Left => location.0 -= 1,
    }
}

fn spawn_beam(
    cave: &Vec<Vec<char>>,
    energy_matrix: &mut Vec<Vec<bool>>,
    known_paths: &mut HashSet<(Location, Direction)>,
    starting_location: Location,
    starting_direction: Direction,
) {
    let mut location = starting_location;
    let mut direction = starting_direction;
    let mut symbol: char;

    // Are we still within the bounds of the cave?
    while location.0 >= 0
        && location.1 >= 0
        && location.0 < cave[0].len() as i8
        && location.1 < cave.len() as i8
    {
        if known_paths.contains(&(location, direction)) {
            // Another beam has already traveled this path,
            // so we can stop here
            return;
        }

        // Mark this path as traveled
        known_paths.insert((location, direction));

        // Mark this location as energized
        energy_matrix[location.1 as usize][location.0 as usize] = true;

        symbol = cave[location.1 as usize][location.0 as usize];

        if symbol != '.' {
            // We hit a splitter, so we may need to
            // change direction and/or spawn a new beam
            maybe_change_direction_and_spawn(
                cave,
                energy_matrix,
                known_paths,
                location,
                &mut direction,
                symbol,
            );
        }

        move_in_direction(&mut location, &mut direction)
    }
}

fn count_energized(cave: &Vec<Vec<char>>, location: Location, direction: Direction) -> usize {
    let mut energy_matrix = vec![vec![false; cave[0].len()]; cave.len()];
    let mut known_paths: HashSet<(Location, Direction)> = HashSet::new();

    spawn_beam(
        &cave,
        &mut energy_matrix,
        &mut known_paths,
        location,
        direction,
    );

    energy_matrix
        .iter()
        .map(|row| row.iter().filter(|&point| *point).count())
        .sum()
}

fn part1(input: &str) -> usize {
    let cave = input
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();

    count_energized(&cave, (0, 0), Direction::Right)
}

fn part2(input: &str) -> usize {
    let cave = input
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let mut max_energized = 0;

    for i in 0..cave[0].len() {
        max_energized = max_energized.max(count_energized(&cave, (i as i8, 0), Down));
        max_energized =
            max_energized.max(count_energized(&cave, (i as i8, cave.len() as i8 - 1), Up));
    }

    for i in 0..cave.len() {
        max_energized = max_energized.max(count_energized(&cave, (0, i as i8), Right));
        max_energized = max_energized.max(count_energized(
            &cave,
            (cave[0].len() as i8 - 1, i as i8),
            Left,
        ));
    }

    max_energized
}

fn main() {
    let input = include_str!("../../input/16.txt").trim_end();

    println!("part 1: {}", part1(input));
    println!("part 2: {}", part2(input));
}
