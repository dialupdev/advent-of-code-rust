// https://adventofcode.com/2023/day/16

use std::collections::HashSet;
use Direction::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Location {
    x: i8,
    y: i8,
}

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
            spawn_beam(cave, energy_matrix, known_paths, location, Direction::Left);
            *direction = Right;
        }
        (Up, '\\') => *direction = Left,

        // Right
        (Right, '|') => {
            spawn_beam(cave, energy_matrix, known_paths, location, Direction::Up);
            *direction = Down;
        }
        (Right, '/') => *direction = Up,
        (Right, '-') => *direction = Right,
        (Right, '\\') => *direction = Down,

        // Down
        (Down, '|') => *direction = Down,
        (Down, '/') => *direction = Left,
        (Down, '-') => {
            spawn_beam(cave, energy_matrix, known_paths, location, Direction::Right);
            *direction = Left;
        }
        (Down, '\\') => *direction = Right,

        // Left
        (Left, '|') => {
            spawn_beam(cave, energy_matrix, known_paths, location, Direction::Down);
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
        Up => location.y -= 1,
        Right => location.x += 1,
        Down => location.y += 1,
        Left => location.x -= 1,
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
    while location.x >= 0
        && location.y >= 0
        && location.x < cave[0].len() as i8
        && location.y < cave.len() as i8
    {
        if known_paths.contains(&(location, direction)) {
            // Another beam has already traveled this path,
            // so we can stop here
            return;
        }

        // Mark this path as traveled
        known_paths.insert((location, direction));

        // Mark this location as energized
        energy_matrix[location.y as usize][location.x as usize] = true;

        symbol = cave[location.y as usize][location.x as usize];

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

fn part1(input: &str) -> usize {
    let cave = input
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();
    let mut energy_matrix = vec![vec![false; cave[0].len()]; cave.len()];
    let mut known_paths: HashSet<(Location, Direction)> = HashSet::new();

    spawn_beam(
        &cave,
        &mut energy_matrix,
        &mut known_paths,
        Location { x: 0, y: 0 },
        Direction::Right,
    );

    energy_matrix
        .iter()
        .map(|rown| rown.iter().filter(|&light| *light).count())
        .sum()
}

fn main() {
    let input = include_str!("../../input/16.txt").trim_end();

    println!("part 1: {}", part1(input));
}
