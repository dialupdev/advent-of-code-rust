// https://adventofcode.com/2016/day/9

use regex::Regex;

fn part1(input: &str) -> usize {
    let re = Regex::new(r"\((\d+)x(\d+)\)").unwrap();

    let mut length = 0;
    let mut index = 0;

    loop {
        let next_section = &input[index..];
        println!("next_section: {}", next_section);
        let caps = re.captures(next_section);

        if let Some(caps) = caps {
            let marker_start = caps.get(0).unwrap().start();
            let marker_end = caps.get(0).unwrap().end();
            println!("start: {}, end: {}", marker_start, marker_end);

            // prob can remove this if statement
            // Count any characters before the start of the next marker
            if marker_start > 0 {
                length += marker_start;
            }

            // parse match data and add to length
            let count = &caps[1].parse::<usize>().unwrap();
            let repeat = &caps[2].parse::<usize>().unwrap();
            println!("count: {}, repeat: {}", &caps[1], &caps[2]);

            length += count * repeat;

            // increment index by match size + match data length
            index += marker_end + count;

            continue;
        } else {
            // no match - add any leftover chars and exit
            length += next_section.len();
            break;
        }
    }

    length
}

fn main() {
    let input = include_str!("../../input/09.txt").trim_end();

    println!("part 1: {}", part1(input));
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("../../input/09.txt");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT.trim_end()), 74532);
    }
}
