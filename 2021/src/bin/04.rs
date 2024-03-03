// https://adventofcode.com/2021/day/4

use std::collections::HashSet;

type Board = Vec<HashSet<u8>>;

fn create_boards(input: &str) -> Vec<Board> {
    // Boards are represented as sets of possible wins - both rows and columns
    // Every cell gets duplicated, but that's ok
    input
        .split("\n\n")
        .skip(1)
        .map(|board_data| {
            // Build rows
            let rows_vec = board_data
                .split('\n')
                .map(|row| {
                    row.split_whitespace()
                        .map(|cell| cell.parse::<u8>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();

            // Convert each row to a HashSet
            let mut board: Board = rows_vec
                .iter()
                .map(|row_vec| {
                    let bb = HashSet::from_iter(row_vec.clone());
                    bb
                })
                .collect();

            // Add columns by transposing the rows 90 degrees
            // This transposition code only works with square matrices
            let mut columns: Vec<HashSet<u8>> =
                vec![HashSet::with_capacity(rows_vec.len()); rows_vec.len()];

            rows_vec.iter().for_each(|row_vec| {
                row_vec.iter().enumerate().for_each(|(j, cell)| {
                    columns[j].insert(*cell);
                })
            });

            board.append(&mut columns);

            board
        })
        .collect::<Vec<_>>()
}

fn mark_number_and_check_for_win(board: &mut Board, turn: u8) -> bool {
    for line in board.iter_mut() {
        line.remove(&turn);

        if line.is_empty() {
            return true;
        }
    }

    false
}

fn calculate_score(board: &Board, turn: u8) -> u32 {
    // We only want the first 5 lines (the rows).
    // If we took the columns too, we'd have duplicate cells.
    let leftover_sum = board
        .iter()
        .take(5)
        .map(|line| line.iter().map(|cell| *cell as u32).sum::<u32>())
        .sum::<u32>();

    leftover_sum * turn as u32
}

fn part1(input: &str) -> u32 {
    let mut boards = create_boards(input);

    let turns = input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|turn| turn.parse::<u8>().unwrap());

    for turn in turns {
        for board in boards.iter_mut() {
            if mark_number_and_check_for_win(board, turn) {
                // This board wins!
                return calculate_score(board, turn);
            }
        }
    }

    0
}

fn main() {
    let input = include_str!("../../input/04.txt").trim_end();

    println!("part 1: {}", part1(input));
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("../../input/04.txt");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT.trim_end()), 67716);
    }
}
