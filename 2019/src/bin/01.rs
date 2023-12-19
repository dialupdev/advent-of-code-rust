// https://adventofcode.com/2019/day/1

fn fuel_requirement(mass_or_fuel: u32) -> u32 {
    let fuel = mass_or_fuel / 3 - 2;

    // 6 / 3 - 2 = 0
    if fuel > 6 {
        fuel + fuel_requirement(fuel)
    } else {
        fuel
    }
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| line.parse::<u32>().unwrap() / 3 - 2)
        .sum()
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| fuel_requirement(line.parse::<u32>().unwrap()))
        .sum()
}

fn main() {
    let input = include_str!("../../input/01.txt").trim_end();

    println!("part 1: {}", part1(input));
    println!("part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("../../input/01.txt");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT.trim_end()), 3401852);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT.trim_end()), 5099916);
    }
}
