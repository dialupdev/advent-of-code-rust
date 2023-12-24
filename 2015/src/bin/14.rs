// https://adventofcode.com/2015/day/14

use regex::Regex;

#[derive(Debug)]
struct Reindeer {
    fly_rate: u8,
    fly_time: u8,
    rest_time: u8,
    timer: u8,
    is_flying: bool,
    distance: u16,
    score: u16,
}

impl Reindeer {
    fn new(fly_rate: u8, fly_time: u8, rest_time: u8) -> Reindeer {
        Reindeer {
            fly_rate,
            fly_time,
            rest_time,
            is_flying: true,
            distance: 0,
            timer: 0,
            score: 0,
        }
    }

    fn tick(self: &mut Self) {
        self.timer += 1;

        if self.is_flying {
            self.distance += self.fly_rate as u16;

            if self.timer == self.fly_time {
                self.timer = 0;
                self.is_flying = false;
            }
        } else if self.timer == self.rest_time {
            self.timer = 0;
            self.is_flying = true;
        }
    }
}

fn parse_input(input: &str) -> Vec<Reindeer> {
    let re = Regex::new(r"fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+)").unwrap();

    let mut reindeer: Vec<Reindeer> = vec![];

    input.lines().for_each(|line| {
        let caps = re.captures(line).unwrap();
        let fly_rate = caps[1].parse().unwrap();
        let fly_time = caps[2].parse().unwrap();
        let rest_time = caps[3].parse().unwrap();

        reindeer.push(Reindeer::new(fly_rate, fly_time, rest_time));
    });

    reindeer
}

fn part1(input: &str) -> u16 {
    let mut reindeer = parse_input(input);

    for _ in 0..2503 {
        for deer in &mut reindeer {
            deer.tick();
        }
    }

    reindeer.iter().map(|deer| deer.distance).max().unwrap()
}

fn part2(input: &str) -> u16 {
    let mut reindeer = parse_input(input);

    for _ in 0..2503 {
        let mut max_distance = 0;
        let mut max_distance_reindeer: Vec<&mut Reindeer> = vec![];

        for deer in &mut reindeer {
            deer.tick();

            if deer.distance > max_distance {
                max_distance = deer.distance;

                max_distance_reindeer.clear();
                max_distance_reindeer.push(deer);
            } else if deer.distance == max_distance {
                max_distance_reindeer.push(deer);
            }
        }

        max_distance_reindeer
            .iter_mut()
            .for_each(|deer| deer.score += 1);
    }

    reindeer.iter().map(|deer| deer.score).max().unwrap()
}

fn main() {
    let input = include_str!("../../input/14.txt").trim_end();

    println!("part 1: {}", part1(input));
    println!("part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("../../input/14.txt");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT.trim_end()), 2660);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT.trim_end()), 1256);
    }
}
