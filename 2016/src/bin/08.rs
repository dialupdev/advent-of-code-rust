// https://adventofcode.com/2016/day/8

use regex::Regex;
use std::collections::VecDeque;

const SCREEN_WIDTH: usize = 50;
const SCREEN_HEIGHT: usize = 6;

fn draw_rect(screen: &mut Vec<VecDeque<bool>>, width: usize, height: usize) {
    for i in 0..height {
        for j in 0..width {
            screen[i][j] = true;
        }
    }
}

fn rotate_row(screen: &mut Vec<VecDeque<bool>>, i: usize, amount: usize) {
    screen[i].rotate_right(amount);
}

fn rotate_column(screen: &mut Vec<VecDeque<bool>>, j: usize, amount: usize) {
    let mut column: VecDeque<_> = (0..SCREEN_HEIGHT).map(|i| screen[i][j]).collect();

    column.rotate_right(amount);

    for i in 0..SCREEN_HEIGHT {
        screen[i][j] = column[i];
    }
}

fn parse_input(input: &str) -> Vec<VecDeque<bool>> {
    let mut screen = vec![VecDeque::from(vec![false; SCREEN_WIDTH]); SCREEN_HEIGHT];

    let rect_re = Regex::new(r"rect (\d+)x(\d+)").unwrap();
    let rotate_re = Regex::new(r"rotate (row|column) \w=(\d+) by (\d+)").unwrap();

    for line in input.lines() {
        if let Some(rect) = rect_re.captures(line) {
            let width = rect[1].parse().unwrap();
            let height = rect[2].parse().unwrap();

            draw_rect(&mut screen, width, height);

            continue;
        }

        let rotate = rotate_re.captures(line).unwrap();
        let orientation = &rotate[1];
        let index = rotate[2].parse().unwrap();
        let amount = rotate[3].parse().unwrap();

        match orientation {
            "row" => rotate_row(&mut screen, index, amount),
            "column" => rotate_column(&mut screen, index, amount),
            _ => unreachable!(),
        }
    }

    screen
}

fn render_screen(screen: Vec<VecDeque<bool>>) -> String {
    let mut output = String::with_capacity(SCREEN_WIDTH * SCREEN_HEIGHT + SCREEN_HEIGHT);

    for i in 0..SCREEN_HEIGHT {
        for j in 0..SCREEN_WIDTH {
            if screen[i][j] {
                output.push('#');
            } else {
                output.push('.');
            }
        }

        if i < SCREEN_HEIGHT - 1 {
            output.push('\n');
        }
    }

    output
}

fn part1(input: &str) -> usize {
    let screen = parse_input(input);

    screen
        .iter()
        .map(|row| row.iter().filter(|&pixel| *pixel).count())
        .sum()
}

fn part2(input: &str) -> String {
    let screen = parse_input(input);

    render_screen(screen)
}

fn main() {
    let input = include_str!("../../input/08.txt").trim_end();

    println!("part 1: {}", part1(input));
    println!("part 2:\n{}", part2(input));
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("../../input/08.txt");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT.trim_end()), 110);
    }

    #[test]
    fn part2() {
        assert_eq!(
            super::part2(INPUT.trim_end()),
            "####...##.#..#.###..#..#..##..###..#....#...#..##.
...#....#.#..#.#..#.#.#..#..#.#..#.#....#...#...#.
..#.....#.####.#..#.##...#....#..#.#.....#.#....#.
.#......#.#..#.###..#.#..#....###..#......#.....#.
#....#..#.#..#.#.#..#.#..#..#.#....#......#..#..#.
####..##..#..#.#..#.#..#..##..#....####...#...##.."
        );
    }
}
