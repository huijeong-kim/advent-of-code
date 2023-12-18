use std::collections::HashMap;

pub fn solution(input: String) {
    let result = part1(&input);
    println!("part1: {}", result);
    let result = part2(&input);
    println!("part2: {}", result);
}

const UP: u8 = 0b1000;
const RIGHT: u8 = 0b0100;
const DOWN: u8 = 0b0010;
const LEFT: u8 = 0b0001;
#[derive(Clone, Debug)]
struct Direction(u8);
impl Direction {
    fn has(&self, dir: &Direction) -> bool {
        (self.0 & dir.0) != 0
    }
    fn set(&mut self, dir: &Direction) {
        self.0 |= dir.0
    }
}

#[derive(Clone, PartialEq, Hash, Eq, Debug)]
struct Location(i32, i32);
impl Location {
    fn to_right(&self) -> Location {
        Location(self.0, self.1 + 1)
    }
    fn to_left(&self) -> Location {
        Location(self.0, self.1 - 1)
    }
    fn to_up(&self) -> Location {
        Location(self.0 - 1, self.1)
    }
    fn to_down(&self) -> Location {
        Location(self.0 + 1, self.1)
    }
    fn to_dir(&self, dir: &Direction) -> Location {
        if dir.0 == RIGHT {
            self.to_right()
        } else if dir.0 == LEFT {
            self.to_left()
        } else if dir.0 == UP {
            self.to_up()
        } else if dir.0 == DOWN {
            self.to_down()
        } else {
            unreachable!()
        }
    }
}
struct Map {
    value: Vec<Vec<char>>,
}
impl Map {
    fn is_valid(&self, loc: &Location) -> bool {
        let size = self.size();
        loc.0 >= 0 && loc.0 < size.0 as i32 && loc.1 >= 0 && loc.1 < size.1 as i32
    }

    fn size(&self) -> (usize, usize) {
        (self.value.len(), self.value[0].len())
    }
}

#[derive(Debug)]
struct State {
    location: Location,
    direction: Direction,
}
fn print(map_size: (usize, usize), check: &HashMap<(i32, i32), Direction>) {
    for row in 0..map_size.0 {
        for col in 0..map_size.1 {
            if check.contains_key(&(row as i32, col as i32)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn do_next(loc: &Location, dir: u8, map: &Map) -> Option<State> {
    let dir = Direction { 0: dir };
    let location = loc.to_dir(&dir);

    return if map.is_valid(&location) {
        Some(State {
            location,
            direction: dir,
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

    let start = State {
        location: Location(0, 0),
        direction: Direction { 0: RIGHT },
    };
    count_energized_tiles(&map, start)
}

fn part2(input: &str) -> u64 {
    let map = Map {
        value: input
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>(),
    };

    let mut starting_points = Vec::new();
    let map_size = map.size();
    for col in 0..map_size.1 {
        starting_points.push(State {
            location: Location(0, col as i32),
            direction: Direction(DOWN),
        });
        starting_points.push(State {
            location: Location(map_size.0 as i32 - 1, col as i32),
            direction: Direction(UP),
        });
    }
    for row in 0..map_size.0 {
        starting_points.push(State {
            location: Location(row as i32, 0),
            direction: Direction(RIGHT),
        });
        starting_points.push(State {
            location: Location(row as i32, map_size.1 as i32 - 1),
            direction: Direction(LEFT),
        });
    }

    starting_points
        .into_iter()
        .map(|start| count_energized_tiles(&map, start))
        .max()
        .unwrap()
}
fn count_energized_tiles(map: &Map, start: State) -> u64 {
    let mut beam = HashMap::<Location, Direction>::new();
    let mut next_values = vec![start];

    let direction_map: HashMap<_, _> = HashMap::from([
        (('|', LEFT), vec![UP, DOWN]),
        (('|', RIGHT), vec![UP, DOWN]),
        (('|', UP), vec![UP]),
        (('|', DOWN), vec![DOWN]),
        (('-', UP), vec![LEFT, RIGHT]),
        (('-', DOWN), vec![LEFT, RIGHT]),
        (('-', LEFT), vec![LEFT]),
        (('-', RIGHT), vec![RIGHT]),
        (('/', RIGHT), vec![UP]),
        (('/', UP), vec![RIGHT]),
        (('/', LEFT), vec![DOWN]),
        (('/', DOWN), vec![LEFT]),
        (('\\', RIGHT), vec![DOWN]),
        (('\\', DOWN), vec![RIGHT]),
        (('\\', LEFT), vec![UP]),
        (('\\', UP), vec![LEFT]),
        (('.', LEFT), vec![LEFT]),
        (('.', RIGHT), vec![RIGHT]),
        (('.', UP), vec![UP]),
        (('.', DOWN), vec![DOWN]),
    ]);

    while !next_values.is_empty() {
        let current = next_values.pop().unwrap();

        if let Some(val) = beam.get_mut(&current.location) {
            if val.has(&current.direction) {
                continue;
            } else {
                val.set(&current.direction);
            }
        } else {
            beam.insert(current.location.clone(), current.direction.clone());
        }

        let c = map.value[current.location.0 as usize][current.location.1 as usize];
        let directions = &direction_map[&(c, current.direction.0)];

        for dir in directions {
            if let Some(next) = do_next(&current.location, *dir, &map) {
                next_values.push(next);
            }
        }
    }

    // let map_size = map.size();
    //print(map_size, &beam);

    beam.len() as u64
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

    #[test]
    fn test_part1() {
        let input = crate::read_from_file("inputs/day16.txt");
        let result = part1(&input);
        assert_eq!(result, 6795);
    }

    #[test]
    fn test_part2_with_test_input() {
        let result = part2(TEST_INPUT);
        assert_eq!(result, 51);
    }
}
