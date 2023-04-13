// https://adventofcode.com/2017/day/5

fn parse_input(input: &str) -> Vec<isize> {
    input
        .lines()
        .map(|line| line.parse::<isize>().unwrap())
        .collect()
}

fn part1(input: &str) -> isize {
    let mut nums = parse_input(input);
    let mut i = 0isize;
    let mut count = 0;

    loop {
        if i < 0 || i > nums.len() as isize - 1 {
            break;
        }

        let num = nums[i as usize];
        nums[i as usize] += 1;

        i += num;
        count += 1;
    }

    count
}

fn part2(input: &str) -> isize {
    let mut nums = parse_input(input);
    let mut i = 0isize;
    let mut count = 0;

    loop {
        if i < 0 || i > nums.len() as isize - 1 {
            break;
        }

        let num = nums[i as usize];
        if num >= 3 {
            nums[i as usize] -= 1;
        } else {
            nums[i as usize] += 1;
        }

        i += num;
        count += 1;
    }

    count
}

fn main() {
    let input = include_str!("../../input/05.txt").trim_end();

    println!("part 1: {}", part1(input));
    println!("part 2: {}", part2(input));
}
