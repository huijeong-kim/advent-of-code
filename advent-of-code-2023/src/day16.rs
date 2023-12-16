use std::collections::HashMap;

pub fn solution(input: String) {
    let result = part1(&input);
    println!("part1: {}", result);
}

#[derive(PartialEq, Debug, Hash, Eq, Clone)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

struct Map {
    value: Vec<Vec<char>>,
}
impl Map {
    fn is_valid(&self, loc: (i32, i32)) -> bool {
        let size = self.size();
        loc.0 >= 0 && loc.0 < size.0 as i32 && loc.1 >= 0 && loc.1 < size.1 as i32
    }

    fn size(&self) -> (usize, usize) {
        (self.value.len(), self.value[0].len())
    }
}

#[derive(Debug)]
struct State {
    location: (i32, i32),
    direction: Direction,
}

fn to_right(current: (i32, i32)) -> (i32, i32) {
    (current.0, current.1 + 1)
}
fn to_left(current: (i32, i32)) -> (i32, i32) {
    (current.0, current.1 - 1)
}
fn to_up(current: (i32, i32)) -> (i32, i32) {
    (current.0 - 1, current.1)
}
fn to_down(current: (i32, i32)) -> (i32, i32) {
    (current.0 + 1, current.1)
}
fn to_dir(current: (i32, i32), dir: &Direction) -> (i32, i32) {
    match dir {
        Direction::Right => to_right(current),
        Direction::Left => to_left(current),
        Direction::Up => to_up(current),
        Direction::Down => to_down(current),
    }
}

fn print(check: &Vec<Vec<bool>>) {
    check.iter().for_each(|row| {
        row.iter()
            .for_each(|r| if *r == true { print!("#") } else { print!(".") });
        println!();
    });
}

fn do_next(loc: (i32, i32), dir: &Direction, map: &Map) -> Option<State> {
    let location = to_dir(loc, dir);

    return if map.is_valid(location) {
        Some(State {
            location,
            direction: dir.clone(),
        })
    } else {
        None
    };
}

fn part1(input: &str) -> u64 {
    let map = Map {
        value: input
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>(),
    };

    let map_size = map.size();
    let mut check = vec![Vec::new(); map_size.0];
    check
        .iter_mut()
        .for_each(|row| row.resize(map_size.1, false));

    let mut next_values = vec![State {
        location: (0, 0),
        direction: Direction::Right,
    }];

    let direction_map: HashMap<_, _> = HashMap::from([
        (('|', Direction::Left), vec![Direction::Up, Direction::Down]),
        (
            ('|', Direction::Right),
            vec![Direction::Up, Direction::Down],
        ),
        (('|', Direction::Up), vec![Direction::Up]),
        (('|', Direction::Down), vec![Direction::Down]),
        (
            ('-', Direction::Up),
            vec![Direction::Left, Direction::Right],
        ),
        (
            ('-', Direction::Down),
            vec![Direction::Left, Direction::Right],
        ),
        (('-', Direction::Left), vec![Direction::Left]),
        (('-', Direction::Right), vec![Direction::Right]),
        (('/', Direction::Right), vec![Direction::Up]),
        (('/', Direction::Up), vec![Direction::Right]),
        (('/', Direction::Left), vec![Direction::Down]),
        (('/', Direction::Down), vec![Direction::Left]),
        (('\\', Direction::Right), vec![Direction::Down]),
        (('\\', Direction::Down), vec![Direction::Right]),
        (('\\', Direction::Left), vec![Direction::Up]),
        (('\\', Direction::Up), vec![Direction::Left]),
        (('.', Direction::Left), vec![Direction::Left]),
        (('.', Direction::Right), vec![Direction::Right]),
        (('.', Direction::Up), vec![Direction::Up]),
        (('.', Direction::Down), vec![Direction::Down]),
    ]);

    while !next_values.is_empty() {
        // how to stop this...?? when there's a loop
        let current = next_values.pop().unwrap();
        check[current.location.0 as usize][current.location.1 as usize] = true;

        print(&check);
        println!();

        let c = map.value[current.location.0 as usize][current.location.1 as usize];
        let directions = &direction_map[&(c, current.direction)];

        for dir in directions {
            if let Some(next) = do_next(current.location, dir, &map) {
                next_values.push(next);
            }
        }
    }

    check.iter().flatten().filter(|&c| *c == true).count() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = ".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....";

    #[test]
    fn test_part1_with_test_input() {
        let result = part1(TEST_INPUT);
        assert_eq!(result, 46);
    }
}
