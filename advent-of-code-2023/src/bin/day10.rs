use std::fmt::{Debug, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn day10_input() -> Vec<String> {
    let file = File::open("inputs/day10.txt").unwrap();
    BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect()
}

#[derive(Clone)]
enum Direction {
    North,
    South,
    West,
    East,
}
#[derive(Clone, PartialEq)]
struct Location(i64, i64);
impl Location {
    fn next(&self, dir: Direction) -> Location {
        match dir {
            Direction::North => Location(self.0 - 1, self.1),
            Direction::South => Location(self.0 + 1, self.1),
            Direction::West => Location(self.0, self.1 - 1),
            Direction::East => Location(self.0, self.1 + 1),
        }
    }
}
impl Debug for Location {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

struct Node {
    is_start: bool,
    loc: Location,
    next: Vec<Location>,
}
impl Node {
    fn is_connected_to(&self, loc: &Location) -> bool {
        self.next.contains(loc)
    }
    fn is_left_top_corner(&self) -> bool {
        self.next.contains(&self.loc.next(Direction::East))
            && self.next.contains(&self.loc.next(Direction::South))
    }
}
impl Debug for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.loc)?;
        if self.next.len() != 0 {
            write!(f, "->")?;
        }
        if let Some(next) = self.next.get(0) {
            write!(f, " {:?}", next)?;
        }
        if let Some(next) = self.next.get(1) {
            write!(f, " {:?}", next)?;
        }
        write!(f, "")
    }
}
struct Map {
    rows: i64,
    cols: i64,
    start: Location,
    nodes: Vec<Vec<Node>>,
}
impl Map {
    fn next_to_start(&self) -> Vec<Location> {
        let start_loc = &self.nodes[self.start.0 as usize][self.start.1 as usize].loc;
        let directions = vec![
            Direction::West,
            Direction::East,
            Direction::North,
            Direction::South,
        ];
        directions
            .iter()
            .filter(|&dir| {
                let loc = start_loc.next(dir.clone());
                if self.is_valid(&loc) {
                    let next_node = &self.nodes[loc.0 as usize][loc.1 as usize];
                    next_node.is_connected_to(start_loc)
                } else {
                    false
                }
            })
            .map(|dir| start_loc.next(dir.clone()))
            .collect::<Vec<_>>()
    }

    fn replace_start(&mut self) {
        self.nodes[self.start.0 as usize][self.start.1 as usize].next = self.next_to_start();
    }

    fn is_valid(&self, loc: &Location) -> bool {
        loc.0 >= 0 && loc.0 < self.rows && loc.1 >= 0 && loc.1 < self.cols
    }
}

fn parse_input(input: &Vec<String>) -> Map {
    let nodes: Vec<Vec<_>> = input
        .iter()
        .enumerate()
        .map(move |(row_idx, line)| {
            line.chars()
                .enumerate()
                .map(move |(col_idx, c)| {
                    let current = Location(row_idx as i64, col_idx as i64);
                    let next = match &c {
                        '|' => vec![
                            current.next(Direction::North),
                            current.next(Direction::South),
                        ],
                        '-' => vec![current.next(Direction::West), current.next(Direction::East)],
                        'L' => vec![
                            current.next(Direction::North),
                            current.next(Direction::East),
                        ],
                        'J' => vec![
                            current.next(Direction::North),
                            current.next(Direction::West),
                        ],
                        '7' => vec![
                            current.next(Direction::West),
                            current.next(Direction::South),
                        ],
                        'F' => vec![
                            current.next(Direction::South),
                            current.next(Direction::East),
                        ],
                        'S' => vec![],
                        _ => vec![],
                    };
                    let is_start = match &c {
                        'S' => true,
                        _ => false,
                    };

                    Node {
                        is_start,
                        loc: current,
                        next,
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let start = nodes.iter().flatten().find(|node| node.is_start).unwrap();

    Map {
        rows: input.len() as i64,
        cols: input[0].len() as i64,
        start: start.loc.clone(),
        nodes,
    }
}

fn create_steps_vector(rows: i64, cols: i64) -> Vec<Vec<i64>> {
    let mut val = vec![Vec::new(); rows as usize];
    val.iter_mut().for_each(|row| {
        row.resize(cols as usize, -1);
    });

    val
}

fn update_steps(cur: Location, map: &Map, steps: &mut Vec<Vec<i64>>) {
    let mut next = vec![cur];

    while let Some(cur) = next.pop() {
        let mut temp_next = Vec::new();

        let cur_steps = steps[cur.0 as usize][cur.1 as usize];
        let cur_next = &map.nodes[cur.0 as usize][cur.1 as usize].next;
        cur_next.iter().for_each(|loc| {
            if steps[loc.0 as usize][loc.1 as usize] == -1
                || steps[loc.0 as usize][loc.1 as usize] > cur_steps + 1
            {
                steps[loc.0 as usize][loc.1 as usize] = cur_steps + 1;
                temp_next.push(loc.clone());
            }
        });

        //steps.iter().for_each(|row| println!("{:?}", row) );
        next.append(&mut temp_next);
    }
}

fn part1(input: &Vec<String>) -> u64 {
    let mut map = parse_input(input);
    map.replace_start();

    let mut steps = create_steps_vector(map.rows, map.cols);
    steps[map.start.0 as usize][map.start.1 as usize] = 0;

    update_steps(map.start.clone(), &map, &mut steps);

    *steps.iter().flatten().max().unwrap() as u64
}

struct Rectangle {
    left_top: Location,
    right_bottom: Location,
}
impl Rectangle {
    fn size(&self) -> u64 {
        ((self.right_bottom.1 - self.left_top.1) * (self.right_bottom.0 - self.left_top.0)) as u64
    }
}
fn find_rectangle(start_node: &Location, map: &Map) -> Option<Rectangle> {
    todo!()
}

fn part2(input: &Vec<String>) -> u64 {
    let mut map = parse_input(input);
    map.replace_start();

    let left_top_corners = map
        .nodes
        .iter()
        .map(|row| {
            row.iter()
                .filter(|node| node.is_left_top_corner())
                .map(|node| node.loc.clone())
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<Vec<_>>();

    let rectangles = left_top_corners
        .iter()
        .map(|node| find_rectangle(node, &map))
        .collect::<Vec<_>>()
        .filter(|rec| rec.is_some())
        .map(|rec| rec.unwrap())
        .collect::<Vec<_>>();
    todo!()
}

fn main() {
    let input = day10_input();
    let result = part1(&input);
    println!("result: {}", result);

    let result = part2(&input);
    println!("result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn part1_test_input() -> Vec<String> {
        let input = ".....
.S-7.
.|.|.
.L-J.
.....";
        input.lines().map(|line| line.to_string()).collect()
    }

    fn part2_test_input() -> Vec<String> {
        let input = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

        input.lines().map(|line| line.to_string()).collect()
    }

    #[test]
    fn test_part1_with_test_input() {
        let input = part1_test_input();
        let result = part1(&input);

        assert_eq!(result, 4);
    }

    #[test]
    fn test_part1() {
        let input = day10_input();
        let result = part1(&input);
        assert_eq!(result, 6890);
    }

    #[test]
    fn test_part2() {
        let input = part2_test_input();
        let result = part2(&input);
        assert_eq!(result, 4);
    }
}
