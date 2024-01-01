// https://adventofcode.com/2020/day/8

fn run_program(program: &Vec<&str>) -> (Vec<usize>, i32, bool) {
    let mut exited = false;
    let mut pointer = 0;
    let mut acc = 0;

    let mut visited_locations = vec![pointer];

    loop {
        let line = program[pointer];
        let mut parts = line.split_whitespace();
        let command = parts.next().unwrap();
        let value = parts.next().unwrap().parse::<i32>().unwrap();

        match command {
            "acc" => {
                acc += value;
                pointer += 1
            }
            "jmp" => pointer = (pointer as i32 + value) as usize,
            "nop" => pointer += 1,
            _ => unreachable!(),
        }

        if visited_locations.contains(&pointer) {
            break;
        } else {
            visited_locations.push(pointer);
        }

        if pointer >= program.len() {
            exited = true;
            break;
        }
    }

    (visited_locations, acc, exited)
}

fn part1(input: &str) -> i32 {
    let program = input.lines().collect::<Vec<&str>>();

    let (_, acc, _) = run_program(&program);

    acc
}

fn part2(input: &str) -> i32 {
    let program = input.lines().collect::<Vec<&str>>();

    let (visited_locations, _, _) = run_program(&program);

    // Loop though the commands that were actually executed,
    // and try changing each one to see if it fixes the program
    for pointer in visited_locations.iter() {
        let line = program[*pointer];

        let mut parts = line.split_whitespace();
        let command = parts.next().unwrap();
        let value = parts.next().unwrap().parse::<i32>().unwrap();

        let new_command = match command {
            "jmp" => format!("nop {}", value),
            "nop" => format!("jmp {}", value),
            _ => continue,
        };

        let mut new_program = program.clone();
        new_program[*pointer] = &new_command;

        let (_, acc, exited) = run_program(&new_program);

        if exited {
            return acc;
        }
    }

    0
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
        assert_eq!(super::part1(INPUT.trim_end()), 1782);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT.trim_end()), 797);
    }
}
