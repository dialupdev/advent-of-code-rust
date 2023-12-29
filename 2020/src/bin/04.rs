// https://adventofcode.com/2020/day/4

use once_cell::sync::Lazy;
use regex::Regex;

static HGT_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(\d+)(cm|in)$").unwrap());
static HCL_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^#[0-9a-f]{6}$").unwrap());
static PID_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\d{9}$").unwrap());

fn has_all_keys(keys: Vec<&str>) -> bool {
    ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .iter()
        .all(|key| keys.contains(key))
}

fn validate_numeric_range(value: &str, min: u16, max: u16) -> bool {
    let num = value.parse::<u16>();

    match num {
        Ok(num) => num >= min && num <= max,
        Err(_) => false,
    }
}

fn validate_field(key: &str, value: &str) -> bool {
    match key {
        "byr" => validate_numeric_range(value, 1920, 2002),
        "iyr" => validate_numeric_range(value, 2010, 2020),
        "eyr" => validate_numeric_range(value, 2020, 2030),
        "hgt" => match HGT_RE.captures(value) {
            Some(caps) => {
                let num = &caps[1];
                let unit = &caps[2];

                match unit {
                    "cm" => validate_numeric_range(num, 150, 193),
                    "in" => validate_numeric_range(num, 59, 76),
                    _ => false,
                }
            }
            None => false,
        },
        "hcl" => HCL_RE.is_match(value),
        "ecl" => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&value),
        "pid" => PID_RE.is_match(value),
        _ => true,
    }
}

fn part1(input: &str) -> usize {
    input
        .split("\n\n")
        .filter(|passport| {
            let keys = passport
                .split_whitespace()
                .map(|field| field.split(':').next().unwrap())
                .collect::<Vec<_>>();

            has_all_keys(keys)
        })
        .count()
}

fn part2(input: &str) -> usize {
    input
        .split("\n\n")
        .filter(|passport| {
            let mut keys = vec![];

            passport.split_whitespace().all(|field| {
                let mut split = field.split(':');
                let key = split.next().unwrap();
                let value = split.next().unwrap();

                keys.push(key);

                validate_field(key, value)
            }) && has_all_keys(keys)
        })
        .count()
}

fn main() {
    let input = include_str!("../../input/04.txt").trim_end();

    println!("part 1: {}", part1(input));
    println!("part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("../../input/04.txt");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT.trim_end()), 235);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT.trim_end()), 194);
    }
}
