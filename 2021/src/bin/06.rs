// https://adventofcode.com/2021/day/6

fn part1(input: &str) -> usize {
    let mut fishes = input
        .split(",")
        .map(|n| n.parse::<u16>().unwrap())
        .collect::<Vec<u16>>();

    for _ in 0..80 {
        let mut new_fishes = vec![];

        fishes.iter_mut().for_each(|fish| {
            if *fish == 0 {
                *fish = 6;
                new_fishes.push(8);
            } else {
                *fish -= 1;
            }
        });

        fishes.append(&mut new_fishes);
    }

    fishes.len()
}

fn main() {
    let input = include_str!("../../input/06.txt").trim_end();

    println!("part 1: {}", part1(input));
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("../../input/06.txt");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT.trim_end()), 360268);
    }
}
