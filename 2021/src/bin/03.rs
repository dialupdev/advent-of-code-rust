// https://adventofcode.com/2021/day/3

fn get_frequency_at_index(numbers: &Vec<&str>, index: usize) -> i16 {
    // There are only two possible values (0 or 1),
    // so we can use a single number to track digit frequency
    let mut frequency = 0;

    numbers.iter().for_each(|line| {
        let c = line.chars().nth(index).unwrap();

        if c == '0' {
            frequency -= 1;
        } else if c == '1' {
            frequency += 1;
        }
    });

    frequency
}

fn frequency_to_gamma(f: &i16) -> char {
    if *f >= 0 {
        '1'
    } else {
        '0'
    }
}

fn frequency_to_epsilon(f: &i16) -> char {
    if *f >= 0 {
        '0'
    } else {
        '1'
    }
}

fn get_gamma_rate(frequencies: &Vec<i16>) -> u32 {
    let binary = frequencies
        .iter()
        .map(frequency_to_gamma)
        .collect::<String>();

    u32::from_str_radix(&binary, 2).unwrap()
}

fn get_epsilon_rate(frequencies: &Vec<i16>) -> u32 {
    let binary = frequencies
        .iter()
        .map(frequency_to_epsilon)
        .collect::<String>();

    u32::from_str_radix(&binary, 2).unwrap()
}

fn get_oxygen_generator_rating(numbers: &Vec<&str>) -> u32 {
    let mut numbers = numbers.clone();
    let mut i = 0;

    while numbers.len() > 1 {
        let frequency = get_frequency_at_index(&numbers, i);
        let char = frequency_to_gamma(&frequency);

        numbers.retain(|&n| n.chars().nth(i).unwrap() == char);

        i += 1;
    }

    u32::from_str_radix(numbers.first().unwrap(), 2).unwrap()
}

fn get_co2_scrubber_rating(numbers: &Vec<&str>) -> u32 {
    let mut numbers = numbers.clone();
    let mut i = 0;

    while numbers.len() > 1 {
        let frequency = get_frequency_at_index(&numbers, i);
        let char = frequency_to_epsilon(&frequency);

        numbers.retain(|&n| n.chars().nth(i).unwrap() == char);

        i += 1;
    }

    u32::from_str_radix(numbers.first().unwrap(), 2).unwrap()
}

fn part1(input: &str) -> u32 {
    // There are only two possible values per digit (0 or 1),
    // so we can use single numbers to track digit frequency
    let mut frequencies = vec![0; input.lines().next().unwrap().len()];

    input.lines().for_each(|line| {
        line.chars().enumerate().for_each(|(i, c)| {
            if c == '0' {
                frequencies[i] -= 1;
            } else if c == '1' {
                frequencies[i] += 1;
            }
        });
    });

    get_gamma_rate(&frequencies) * get_epsilon_rate(&frequencies)
}

fn part2(input: &str) -> u32 {
    let numbers = input.lines().collect();

    get_oxygen_generator_rating(&numbers) * get_co2_scrubber_rating(&numbers)
}

fn main() {
    let input = include_str!("../../input/03.txt").trim_end();

    println!("part 1: {}", part1(input));
    println!("part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("../../input/03.txt");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT.trim_end()), 2595824);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT.trim_end()), 2135254);
    }
}
