// https://adventofcode.com/2019/day/2

fn part1(input: &str) -> u32 {
    let mut program = input
        .split(',')
        .map(|num| num.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    // Restore 1202 program alarm state
    program[1] = 12;
    program[2] = 2;

    let mut pointer = 0;

    loop {
        let opcode = program[pointer];

        println!("opcode is {}", opcode);

        // 99 means exit
        if opcode == 99 {
            break;
        }

        let a_address = program[pointer + 1] as usize;
        let a_value = program[a_address];

        let b_address = program[pointer + 2] as usize;
        let b_value = program[b_address];

        let result = match opcode {
            // 1 means add
            1 => a_value + b_value,
            // 2 means multiply
            2 => a_value * b_value,
            _ => unreachable!(),
        };

        let output_address = program[pointer + 3] as usize;

        program[output_address] = result;

        pointer += 4;
    }

    program[0]
}

fn main() {
    let input = include_str!("../../input/02.txt").trim_end();

    println!("part 1: {}", part1(input));
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("../../input/02.txt");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT.trim_end()), 3562624);
    }
}
