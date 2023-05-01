// https://adventofcode.com/2016/day/3

fn is_valid_triangle(sides: &Vec<u16>) -> bool {
    let max_index = sides
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.cmp(b))
        .unwrap()
        .0;

    let max_side = sides[max_index];
    let other_sides = sides
        .iter()
        .enumerate()
        .filter_map(|(i, side)| if i != max_index { Some(side) } else { None })
        .sum::<u16>();

    other_sides > max_side
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let sides: Vec<u16> = line
                .split_whitespace()
                .map(|side| side.parse::<u16>().unwrap())
                .collect();

            is_valid_triangle(&sides)
        })
        .count()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|matrix| {
            let mut transposed_matrix: Vec<Vec<u16>> = vec![vec![]; matrix.len()];

            // This transposition code only works with square matrices
            matrix.iter().for_each(|row| {
                row.split_whitespace().enumerate().for_each(|(j, side)| {
                    transposed_matrix[j].push(side.parse::<u16>().unwrap());
                })
            });

            transposed_matrix
                .iter()
                .filter(|&sides| is_valid_triangle(sides))
                .count()
        })
        .sum()
}

fn main() {
    let input = include_str!("../../input/03.txt").trim_end();

    println!("part 1: {}", part1(input));
    println!("part 2: {}", part2(input));
}
