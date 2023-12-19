// https://adventofcode.com/2022/day/8

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| char.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect()
}

fn part1(input: &str) -> usize {
    let trees = parse_input(input);
    let mut interior_visibility_matrix = vec![vec![false; trees[0].len()]; trees.len()];

    // All edge trees are visible
    let edge_tree_count = trees[0].len() * 2 + (trees.len() - 2) * 2;

    let mut check_visibility = |highest_so_far: &mut u8, i: usize, j: usize| {
        if trees[i][j] > *highest_so_far {
            *highest_so_far = trees[i][j];
            interior_visibility_matrix[i][j] = true;
        }
    };

    // Look through each row for visible trees
    for i in 1..trees.len() - 1 {
        // Left to right
        let mut highest_so_far = trees[i][0];
        for j in 1..trees[i].len() - 1 {
            check_visibility(&mut highest_so_far, i, j);
        }

        // Right to left
        let mut highest_so_far = trees[i][trees[i].len() - 1];
        for j in (1..trees[i].len() - 1).rev() {
            check_visibility(&mut highest_so_far, i, j);
        }
    }

    // Look through each column for visible trees
    for j in 1..trees[0].len() - 1 {
        // Top to bottom
        let mut highest_so_far = trees[0][j];
        for i in 1..trees.len() - 1 {
            check_visibility(&mut highest_so_far, i, j);
        }

        // Bottom to top
        let mut highest_so_far = trees[trees.len() - 1][j];
        for i in (1..trees.len() - 1).rev() {
            check_visibility(&mut highest_so_far, i, j);
        }
    }

    return edge_tree_count
        + interior_visibility_matrix
            .iter()
            .map(|row| row.iter().filter(|&visible| *visible).count())
            .sum::<usize>();
}

fn part2(input: &str) -> u32 {
    let trees = parse_input(input);
    let mut scenic_score_matrix = vec![vec![0u32; trees[0].len()]; trees.len()];

    // Visit every tree and look left, right, up, and down
    // All edge trees have a score of 0, so we can ignore them
    for i in 1..trees.len() - 1 {
        for j in 1..trees[i].len() - 1 {
            // Look right
            let mut right_score = 0u8;
            for local_j in j + 1..trees[i].len() {
                right_score += 1;
                if trees[i][j] <= trees[i][local_j] {
                    break;
                };
            }

            // Look left
            let mut left_score = 0u8;
            for local_j in (0..j).rev() {
                left_score += 1;
                if trees[i][j] <= trees[i][local_j] {
                    break;
                };
            }

            // Look down
            let mut down_score = 0u8;
            for local_i in i + 1..trees.len() {
                down_score += 1;
                if trees[i][j] <= trees[local_i][j] {
                    break;
                };
            }

            // Look up
            let mut up_score = 0u8;
            for local_i in (0..i).rev() {
                up_score += 1;
                if trees[i][j] <= trees[local_i][j] {
                    break;
                };
            }

            scenic_score_matrix[i][j] =
                right_score as u32 * left_score as u32 * down_score as u32 * up_score as u32;
        }
    }

    *scenic_score_matrix.iter().flatten().max().unwrap()
}

fn main() {
    let input = include_str!("../../input/08.txt").trim_end();

    println!("part 1: {}", part1(input));
    println!("part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("../../input/08.txt");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT.trim_end()), 1719);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT.trim_end()), 590824);
    }
}
