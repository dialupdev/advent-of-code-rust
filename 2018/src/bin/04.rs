// https://adventofcode.com/2018/day/4

use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
enum Record {
    BeginShift(u16),
    FallAsleep(u16),
    WakeUp(u16),
}

fn parse_line(shift_re: &Regex, sleep_re: &Regex, line: &str) -> Record {
    if let Some(caps) = shift_re.captures(line) {
        if let Some(m) = caps.get(1) {
            return Record::BeginShift(m.as_str().parse::<u16>().unwrap());
        }
    }

    let caps = sleep_re.captures(line).unwrap();
    let minute = caps[1].parse::<u16>().unwrap();

    match &caps[2] {
        "falls asleep" => Record::FallAsleep(minute),
        "wakes up" => Record::WakeUp(minute),
        _ => unreachable!(),
    }
}

fn get_guard_naps(input: &str) -> HashMap<u16, HashMap<u16, u16>> {
    let shift_re = Regex::new(r"#(\d+) begins shift").unwrap();
    let sleep_re = Regex::new(r":(\d{2})\] (falls asleep|wakes up)").unwrap();

    let mut lines: Vec<&str> = input.lines().collect();
    lines.sort();

    let mut guard_naps: HashMap<u16, HashMap<u16, u16>> = HashMap::new();
    let mut current_guard_id = 0u16;
    let mut current_nap_start = 0u16;

    lines.iter().for_each(|line| {
        let record = parse_line(&shift_re, &sleep_re, line);

        match record {
            Record::BeginShift(id) => current_guard_id = id,
            Record::FallAsleep(nap_start) => current_nap_start = nap_start,
            Record::WakeUp(nap_end) => {
                for minute in current_nap_start..nap_end {
                    *guard_naps
                        .entry(current_guard_id)
                        .or_default()
                        .entry(minute)
                        .or_default() += 1
                }
            }
        }
    });

    guard_naps
}

fn part1(input: &str) -> u32 {
    let guard_naps = get_guard_naps(input);

    let sleepiest_guard = guard_naps
        .iter()
        .max_by(|a, b| {
            let a_sum = a.1.values().sum::<u16>();
            let b_sum = b.1.values().sum::<u16>();

            a_sum.cmp(&b_sum)
        })
        .unwrap();

    let sleepiest_guard_id = sleepiest_guard.0;
    let sleepiest_minute = sleepiest_guard
        .1
        .iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .unwrap()
        .0;

    (*sleepiest_guard_id as u32) * (*sleepiest_minute as u32)
}

fn part2(input: &str) -> u32 {
    let guard_naps = get_guard_naps(input);

    let most_consistent_guard = guard_naps
        .iter()
        .max_by(|a, b| {
            let a_max = a.1.values().max();
            let b_max = b.1.values().max();

            a_max.cmp(&b_max)
        })
        .unwrap();

    let most_consistent_guard_id = most_consistent_guard.0;
    let sleepiest_minute = most_consistent_guard
        .1
        .iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .unwrap()
        .0;

    (*most_consistent_guard_id as u32) * (*sleepiest_minute as u32)
}

fn main() {
    let input = include_str!("../../input/04.txt").trim_end();

    println!("part 1: {}", part1(input));
    println!("part 2: {}", part2(input));
}
