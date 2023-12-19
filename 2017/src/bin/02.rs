// https://adventofcode.com/2017/day/2

fn parse_rows(input: &str) -> impl Iterator<Item = Vec<u16>> + '_ {
    input.lines().map(|line| {
        line.split('\t')
            .map(|n| n.parse::<u16>().unwrap())
            .collect::<Vec<u16>>()
    })
}

fn part1(input: &str) -> u16 {
    parse_rows(input)
        .map(|row| row.iter().max().unwrap() - row.iter().min().unwrap())
        .sum()
}

fn part2(input: &str) -> u16 {
    parse_rows(input)
        .map(|row| {
            for i in 0..row.len() {
                for j in i + 1..row.len() {
                    let (a, b) = if row[i] < row[j] {
                        (row[j], row[i])
                    } else {
                        (row[i], row[j])
                    };

                    if a % b == 0 {
                        return a / b;
                    }
                }
            }

            0
        })
        .sum()
}

fn main() {
    let input = include_str!("../../input/02.txt").trim_end();

    println!("part 1: {}", part1(input));
    println!("part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("../../input/02.txt");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT.trim_end()), 44216);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT.trim_end()), 320);
    }
}
