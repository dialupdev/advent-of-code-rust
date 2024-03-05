// https://adventofcode.com/2021/day/5

use std::collections::HashMap;

fn count_points(input: &str, skip_diagonals: bool) -> usize {
    // <(x, y), count>
    let mut vents: HashMap<(u16, u16), u8> = HashMap::new();

    input
        .lines()
        .map(|line| {
            let mut split = line.split(" -> ");
            let start = split.next().unwrap();
            let end = split.next().unwrap();

            let mut start_split = start.split(",");
            let x1 = start_split.next().unwrap().parse::<u16>().unwrap();
            let y1 = start_split.next().unwrap().parse::<u16>().unwrap();

            let mut end_split = end.split(",");
            let x2 = end_split.next().unwrap().parse::<u16>().unwrap();
            let y2 = end_split.next().unwrap().parse::<u16>().unwrap();

            (x1, y1, x2, y2)
        })
        .filter(|(x1, y1, x2, y2)| {
            if skip_diagonals {
                x1 == x2 || y1 == y2
            } else {
                true
            }
        })
        .for_each(|(x1, y1, x2, y2)| {
            let mut x = x1;
            let mut y = y1;

            loop {
                *vents.entry((x, y)).or_insert(0) += 1;

                if x == x2 && y == y2 {
                    break;
                }

                x = (x as i16 + (x2 as i16 - x as i16).signum()) as u16;
                y = (y as i16 + (y2 as i16 - y as i16).signum()) as u16;
            }
        });

    vents.iter().filter(|vent| vent.1 > &1).count()
}

fn part1(input: &str) -> usize {
    count_points(input, true)
}

fn part2(input: &str) -> usize {
    count_points(input, false)
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
        assert_eq!(super::part1(INPUT.trim_end()), 7674);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT.trim_end()), 20898);
    }
}
