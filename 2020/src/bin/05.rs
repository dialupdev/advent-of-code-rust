// https://adventofcode.com/2020/day/5

fn parse_input(input: &str) -> Vec<u16> {
    input
        .lines()
        .map(|line| {
            let mut row_start = 0;
            let mut row_end = 127;
            let mut col_start = 0;
            let mut col_end = 7;

            line.chars().for_each(|c| match c {
                'F' => row_end = (row_start + row_end) / 2,
                'B' => row_start = (row_start + row_end) / 2 + 1,
                'L' => col_end = (col_start + col_end) / 2,
                'R' => col_start = (col_start + col_end) / 2 + 1,
                _ => unreachable!(),
            });

            row_start * 8 + col_start
        })
        .collect()
}

fn part1(input: &str) -> u16 {
    *parse_input(input).iter().max().unwrap()
}

fn part2(input: &str) -> u16 {
    let mut ids = parse_input(input);
    ids.sort();

    let mut last_id = ids[0];

    ids.iter()
        .skip(1)
        .find_map(|&id| {
            if id - last_id > 1 {
                Some(id - 1)
            } else {
                last_id = id;
                None
            }
        })
        .unwrap()
}

fn main() {
    let input = include_str!("../../input/05.txt").trim_end();

    println!("part 1: {}", part1(input));
    println!("part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("../../input/05.txt");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT.trim_end()), 930);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT.trim_end()), 515);
    }
}
