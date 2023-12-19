// https://adventofcode.com/2022/day/10

use regex::Regex;

const CRT_WIDTH: usize = 40;
const CRT_HEIGHT: usize = 6;

fn build_commands(input: &str) -> Vec<i16> {
    let re = Regex::new(r"^addx (-?\d+)").unwrap();
    let mut commands = vec![];

    input.lines().for_each(|line| {
        let caps = re.captures(line);

        if let Some(add_command) = caps {
            let value = add_command[1].parse::<i16>().unwrap();
            commands.push(0);
            commands.push(value);
        } else {
            commands.push(0);
        }
    });

    commands
}

fn render_pixel_matrix(pixel_matrix: [[bool; CRT_WIDTH]; CRT_HEIGHT]) -> String {
    let mut output = String::with_capacity(CRT_WIDTH * CRT_HEIGHT + CRT_HEIGHT);

    for i in 0..CRT_HEIGHT {
        for j in 0..CRT_WIDTH {
            if pixel_matrix[i][j] {
                output.push('#');
            } else {
                output.push('.');
            }
        }

        if i < CRT_HEIGHT - 1 {
            output.push('\n');
        }
    }

    output
}

fn part1(input: &str) -> i16 {
    let mut cycle = 0;
    let mut x = 1;
    let mut signal_strength_sum = 0;

    build_commands(input).iter().for_each(|command| {
        cycle += 1;

        if (cycle - 20) % 40 == 0 {
            signal_strength_sum += cycle * x;
        }

        x += command;
    });

    signal_strength_sum
}

fn part2(input: &str) -> String {
    let mut cycle = 0;
    let mut x = 1;
    let mut pixel_matrix = [[false; CRT_WIDTH]; CRT_HEIGHT];

    let mut i = 0;
    let mut j = 0;

    build_commands(input).iter().for_each(|command| {
        cycle += 1;

        if (x - j as i16).abs() <= 1 {
            pixel_matrix[i][j] = true;
        }

        if cycle % 40 == 0 {
            // Start a new row
            j = 0;
            i += 1;
        } else {
            // Move forward within the current row
            j += 1;
        }

        x += command;
    });

    render_pixel_matrix(pixel_matrix)
}

fn main() {
    let input = include_str!("../../input/10.txt").trim_end();

    println!("part 1: {}", part1(input));
    println!("part 2:\n{}", part2(input));
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("../../input/10.txt");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT.trim_end()), 11820);
    }

    #[test]
    fn part2() {
        assert_eq!(
            super::part2(INPUT.trim_end()),
            "####.###....##.###..###..#..#..##..#..#.
#....#..#....#.#..#.#..#.#.#..#..#.#..#.
###..#..#....#.###..#..#.##...#..#.####.
#....###.....#.#..#.###..#.#..####.#..#.
#....#....#..#.#..#.#.#..#.#..#..#.#..#.
####.#.....##..###..#..#.#..#.#..#.#..#."
        );
    }
}
