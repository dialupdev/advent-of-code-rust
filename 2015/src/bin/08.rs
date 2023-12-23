// https://adventofcode.com/2015/day/8

use regex::Regex;

fn part1(input: &str) -> usize {
    let hex_re = Regex::new(r"^\\x[a-fA-F0-9]{2}").unwrap();

    input
        .lines()
        .map(|line| {
            let code_length = line.len();
            let mut mem_length = 0;
            let mut buf = String::new();

            for c in line.chars().skip(1).take(line.len() - 2) {
                // Continue recording an escape sequence
                if buf.len() > 0 {
                    buf.push(c);

                    if buf == r"\\" || buf == r#"\""# || hex_re.is_match(&buf) {
                        // Escape sequence complete
                        buf.clear();
                        mem_length += 1;
                    }

                    continue;
                }

                // Start recording an escape sequence
                // If we're interrupting a partial escape sequence,
                // add its length to the total
                if c == '\\' {
                    mem_length += buf.len();
                    buf.clear();
                    buf.push(c);

                    continue;
                }

                // Regular character
                mem_length += 1;
            }

            // Add any partial escape sequence left over
            mem_length += buf.len();

            code_length - mem_length
        })
        .sum()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let code_length = line.len();
            let mut double_escaped_length = 2; // Start with outer quotes

            for c in line.chars() {
                // Escaped character
                if c == '\\' || c == '"' {
                    double_escaped_length += 2;

                    continue;
                }

                // Regular character
                double_escaped_length += 1;
            }

            double_escaped_length - code_length
        })
        .sum()
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
        assert_eq!(super::part1(INPUT.trim_end()), 1333);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT.trim_end()), 2046);
    }
}
