// https://adventofcode.com/2016/day/2

fn move_in_direction_part_1(location: &mut (u8, u8), direction: char) {
    let grid_size = 3;
    let (x, y) = location;

    match direction {
        'U' => {
            if *y > 0 {
                *y -= 1
            }
        }
        'R' => {
            if *x < grid_size - 1 {
                *x += 1
            }
        }
        'D' => {
            if *y < grid_size - 1 {
                *y += 1
            }
        }
        'L' => {
            if *x > 0 {
                *x -= 1
            }
        }
        _ => unreachable!(),
    }
}

fn move_in_direction_part_2(
    location: &mut (u8, u8),
    direction: char,
    buttons: [[Option<char>; 5]; 5],
) {
    let grid_size = 5;
    let (x, y) = location;

    match direction {
        'U' => {
            if *y > 0 && buttons[*y as usize - 1][*x as usize].is_some() {
                *y -= 1
            }
        }
        'R' => {
            if *x < grid_size - 1 && buttons[*y as usize][*x as usize + 1].is_some() {
                *x += 1
            }
        }
        'D' => {
            if *y < grid_size - 1 && buttons[*y as usize + 1][*x as usize].is_some() {
                *y += 1
            }
        }
        'L' => {
            if *x > 0 && buttons[*y as usize][*x as usize - 1].is_some() {
                *x -= 1
            }
        }
        _ => unreachable!(),
    }
}

fn part1(input: &str) -> String {
    let mut location = (1, 1);

    input
        .lines()
        .map(|line| {
            line.chars().for_each(|direction| {
                move_in_direction_part_1(&mut location, direction);
            });

            // 3x3 grid: Start with the 1-indexed x value,
            // then move down one row for each y value
            let (x, y) = location;
            let num = x + 1 + y * 3;

            char::from_digit(num as u32, 10).unwrap()
        })
        .collect()
}

fn part2(input: &str) -> String {
    let buttons = [
        [None, None, Some('1'), None, None],
        [None, Some('2'), Some('3'), Some('4'), None],
        [Some('5'), Some('6'), Some('7'), Some('8'), Some('9')],
        [None, Some('A'), Some('B'), Some('C'), None],
        [None, None, Some('D'), None, None],
    ];

    let mut location = (0, 2);

    input
        .lines()
        .map(|line| {
            line.chars().for_each(|direction| {
                move_in_direction_part_2(&mut location, direction, buttons);
            });

            let (x, y) = location;
            buttons[y as usize][x as usize].unwrap()
        })
        .collect()
}

fn main() {
    let input = include_str!("../../input/02.txt").trim_end();

    println!("part 1: {}", part1(input));
    println!("part 2: {}", part2(input));
}
