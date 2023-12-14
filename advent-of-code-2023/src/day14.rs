use std::fmt::{Debug, Formatter};

pub fn solution(input: String) {
    let result = part1(&input);
    println!("part1: {}", result);
    let result = part2(&input);
    println!("part2: {}", result);
}

#[derive(Clone, PartialEq)]
enum Rock {
    Rounded,
    Cube,
    Empty,
}
impl Debug for Rock {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Rock::Rounded => 'O',
            Rock::Cube => '#',
            Rock::Empty => '.',
        };
        write!(f, "{value}")
    }
}
fn part1(input: &str) -> u32 {
    let map = create_map(input);
    let new_map = north_tilt(&map);
    get_load(&new_map)
}

fn create_map(input: &str) -> Vec<Vec<Rock>> {
    let num_cols = input.lines().collect::<Vec<_>>()[0].len();
    let num_rows = input.lines().count();

    let mut map = vec![Vec::new(); num_rows];
    map.iter_mut()
        .for_each(|vec| vec.resize(num_cols, Rock::Empty));

    for (row_idx, line) in input.lines().enumerate() {
        line.chars().enumerate().for_each(|(col_idx, value)| {
            let rock = if value == 'O' {
                Rock::Rounded
            } else if value == '#' {
                Rock::Cube
            } else if value == '.' {
                Rock::Empty
            } else {
                unreachable!()
            };

            map[row_idx][col_idx] = rock;
        })
    }

    map
}

fn north_tilt(map: &Vec<Vec<Rock>>) -> Vec<Vec<Rock>> {
    let num_rows = map.len();
    let num_cols = map[0].len();

    let mut new_map = vec![Vec::new(); num_rows];
    new_map
        .iter_mut()
        .for_each(|row| row.resize(num_cols, Rock::Empty));

    for col_idx in 0..num_cols {
        let mut last_row = 0i32;

        for row_idx in 0..num_rows {
            match map[row_idx][col_idx] {
                Rock::Rounded => {
                    new_map[last_row as usize][col_idx] = Rock::Rounded;
                    last_row += 1;
                }
                Rock::Cube => {
                    new_map[row_idx][col_idx] = Rock::Cube;
                    last_row = row_idx as i32 + 1;
                }
                Rock::Empty => {
                    // do nothing
                }
            }
        }
    }

    new_map
}

fn south_tilt(map: &Vec<Vec<Rock>>) -> Vec<Vec<Rock>> {
    let num_rows = map.len();
    let num_cols = map[0].len();

    let mut new_map = vec![Vec::new(); num_rows];
    new_map
        .iter_mut()
        .for_each(|row| row.resize(num_cols, Rock::Empty));

    for col_idx in 0..num_cols {
        let mut last_row = num_rows as i32 - 1;
        for row_idx in (0..num_rows).rev() {
            match map[row_idx][col_idx] {
                Rock::Rounded => {
                    new_map[last_row as usize][col_idx] = Rock::Rounded;
                    last_row -= 1;
                }
                Rock::Cube => {
                    new_map[row_idx][col_idx] = Rock::Cube;
                    last_row = row_idx as i32 - 1;
                }
                Rock::Empty => {
                    // do nothing
                }
            }
        }
    }

    new_map
}

fn east_tilt(map: &Vec<Vec<Rock>>) -> Vec<Vec<Rock>> {
    let num_rows = map.len();
    let num_cols = map[0].len();

    let mut new_map = vec![Vec::new(); num_rows];
    new_map
        .iter_mut()
        .for_each(|row| row.resize(num_cols, Rock::Empty));

    for row_idx in 0..num_rows {
        let mut last_col = num_cols as i32 - 1;

        for col_idx in (0..num_cols).rev() {
            match map[row_idx][col_idx] {
                Rock::Rounded => {
                    new_map[row_idx][last_col as usize] = Rock::Rounded;
                    last_col -= 1;
                }
                Rock::Cube => {
                    new_map[row_idx][col_idx] = Rock::Cube;
                    last_col = col_idx as i32 - 1;
                }
                Rock::Empty => {
                    // do nothing
                }
            }
        }
    }

    new_map
}

fn west_tilt(map: &Vec<Vec<Rock>>) -> Vec<Vec<Rock>> {
    let num_rows = map.len();
    let num_cols = map[0].len();

    let mut new_map = vec![Vec::new(); num_rows];
    new_map
        .iter_mut()
        .for_each(|row| row.resize(num_cols, Rock::Empty));

    for row_idx in 0..num_rows {
        let mut last_col = 0;

        for col_idx in 0..num_cols {
            match map[row_idx][col_idx] {
                Rock::Rounded => {
                    new_map[row_idx][last_col] = Rock::Rounded;
                    last_col += 1;
                }
                Rock::Cube => {
                    new_map[row_idx][col_idx] = Rock::Cube;
                    last_col = col_idx + 1;
                }
                Rock::Empty => {
                    // do nothing
                }
            }
        }
    }

    new_map
}

fn get_load(map: &Vec<Vec<Rock>>) -> u32 {
    let num_rows = map.len();
    let num_cols = map[0].len();

    let mut total_value = 0u32;
    for row_idx in 0..num_rows {
        let mut count = 0;
        for col_idx in 0..num_cols {
            if map[row_idx][col_idx] == Rock::Rounded {
                count += 1;
            }
        }
        total_value += (num_rows as u32 - row_idx as u32) * count;
    }

    total_value
}

fn print_map(map: &Vec<Vec<Rock>>) {
    for row_idx in 0..map.len() {
        for col_idx in 0..map[row_idx].len() {
            print!("{:?} ", map[row_idx][col_idx]);
        }
        println!();
    }
    println!();
}

fn part2(input: &str) -> u32 {
    // Takes too long.. I don't think eq is working properly?
    let mut map = create_map(input);
    let mut last_map = map.clone();

    for _count in 0..1000000000 {
        map = north_tilt(&map);
        map = west_tilt(&map);
        map = south_tilt(&map);
        map = east_tilt(&map);

        if last_map.iter().flatten().eq(map.iter().flatten()) {
            break;
        } else {
            last_map = map.clone();
        }
    }

    print_map(&map);
    get_load(&map)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    #[test]
    fn test_part1_with_test_input() {
        let result = part1(TEST_INPUT);
        assert_eq!(result, 136);
    }

    #[test]
    fn test_part1() {
        let input = crate::read_from_file("inputs/day14.txt");
        let result = part1(&*input);
        assert_eq!(result, 110128);
    }

    #[test]
    fn test_part2_with_test_input() {
        let result = part2(TEST_INPUT);
        assert_eq!(result, 64);
    }

    #[test]
    fn test_equal() {
        let map = create_map(TEST_INPUT);
        let mut new_map = map.clone();
        assert_eq!(map.iter().flatten().eq(new_map.iter().flatten()), true);

        new_map[0].iter_mut().for_each(|col| *col = Rock::Empty);
        assert_eq!(map.iter().flatten().eq(new_map.iter().flatten()), false);
    }
}
