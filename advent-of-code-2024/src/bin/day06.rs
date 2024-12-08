use advent_of_code_2024::read_from_file;

struct Map {
    map: Vec<Vec<char>>,
    visited: Vec<Vec<bool>>,
    guard: GuardStatus,
    rows: usize,
    cols: usize,
}

#[derive(Clone, Debug)]
struct Location {
    row: usize,
    col: usize,
}

impl Location {
    fn get_next(&self, dir: &Direction) -> Location {
        let loc = match dir {
            Direction::Up => (self.row - 1, self.col),
            Direction::Down => (self.row + 1, self.col),
            Direction::Right => (self.row, self.col + 1),
            Direction::Left => (self.row, self.col - 1),
        };
        Location {
            row: loc.0,
            col: loc.1,
        }
    }
}
#[derive(Clone)]
struct GuardStatus {
    loc: Location,
    direction: Direction,
}

const OBSTACLE: char = '#';
const GUARD: char = '^';

const SAFE_LOC: char = '.';

#[derive(Clone)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    fn turn_right(&self) -> Direction {
        match &self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Right => Direction::Down,
            Direction::Left => Direction::Up,
        }
    }
}

impl Map {
    fn new(map: Vec<Vec<char>>) -> Map {
        let mut guard_loc = (0, 0);
        map.iter().enumerate().for_each(|(row, vec)| {
            vec.iter().enumerate().for_each(|(col, char)| {
                if *char == GUARD {
                    guard_loc = (row, col);
                }
            })
        });
        let rows = map.len();
        let cols = map[0].len();
        let mut visited = Vec::new();
        visited.resize(rows, Vec::new());
        visited.iter_mut().for_each(|row| {
            row.resize(cols, false);
        });

        visited[guard_loc.0][guard_loc.1] = true;

        Map {
            map,
            visited,
            guard: GuardStatus {
                loc: Location {
                    row: guard_loc.0,
                    col: guard_loc.1,
                },
                direction: Direction::Up,
            },
            rows,
            cols,
        }
    }
    fn blocked(&self, guard: &GuardStatus) -> bool {
        match &guard.direction {
            Direction::Up => guard.loc.row == 0,
            Direction::Down => guard.loc.row == self.rows - 1,
            Direction::Right => guard.loc.col == self.cols - 1,
            Direction::Left => guard.loc.col == 0,
        }
    }
    fn is_valid(&self, loc: &Location) -> bool {
        loc.row >= 0 && loc.row < self.rows && loc.col >= 0 && loc.col < self.cols
    }

    fn get_next(&mut self, guard: GuardStatus) -> Option<GuardStatus> {
        let next = guard.loc.get_next(&guard.direction);

        if !self.is_valid(&next) {
            return None;
        }

        if self.map[next.row][next.col] == SAFE_LOC || self.map[next.row][next.col] == GUARD {
            self.visited[next.row][next.col] = true;

            Some(GuardStatus {
                loc: next,
                direction: guard.direction.clone(),
            })
        } else if self.map[next.row][next.col] == OBSTACLE {
            Some(GuardStatus {
                loc: guard.loc.clone(),
                direction: guard.direction.turn_right(),
            })
        } else {
            panic!("Should not reach here, {:?}", next)
        }
    }

    fn go(&mut self, mut guard: GuardStatus) -> GuardStatus {
        while !self.blocked(&guard) {
            if let Some(next) = self.get_next(guard.clone()) {
                guard = next;
            } else {
                break;
            }
        }
        guard
    }

    fn traverse(&mut self) {
        let mut cur = self.guard.clone();
        while !self.blocked(&cur) {
            cur = self.go(cur);
        }
    }

    fn count_visited(&self) -> i32 {
        self.visited.iter().fold(0, |acc, row| {
            row.iter().fold(0, |row_acc, val| {
                let mut count = 0;
                if *val == true {
                    count += 1;
                }
                row_acc + count
            }) + acc
        })
    }
    fn print(&self) {
        self.visited.iter().for_each(|row| println!("{:?}", row));
    }
}

fn parse_input(input: &str) -> Map {
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    Map::new(map)
}
fn part1(input: &str) -> i32 {
    let mut map = parse_input(input);
    map.traverse();
    map.count_visited()
}

fn part2(input: &str) -> i32 {
    unimplemented!()
}

fn main() -> Result<(), std::io::Error> {
    let input = read_from_file("inputs/day06.txt")?;
    println!("part1 = {}", part1(&input));
    println!("part2 = {}", part2(&input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 41);
    }

    #[test]
    fn test_part2() { assert_eq!(part2(INPUT), 6); }
}
