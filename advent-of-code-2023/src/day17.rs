use std::cmp::Ordering;
use std::collections::BinaryHeap;

pub fn solution(input: String) {
    let result = part1(&input);
    println!("part1: {}", result);
}

fn part1(input: &str) -> u32 {
    let map: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    let num_rows = map.len();
    let num_cols = map[0].len();

    let mut distances = vec![Vec::new(); num_rows];
    distances
        .iter_mut()
        .for_each(|row| row.resize(num_cols, u32::MAX));
    distances[0][0] = map[0][0];

    let mut queue = BinaryHeap::new();
    queue.push(Node {
        position: Position(0, 0),
        dir: Vec::new(),
        cost: 0,
    });

    while let Some(node) = queue.pop() {
        println!("current updating node: {:?}", node);
        let current_pos = &node.position;
        'dir: for dir in &[
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ] {
            if let Some(adj) = current_pos.adj_to(dir, num_rows, num_cols) {
                if distances[adj.0][adj.1] < map[adj.0][adj.1] {
                    continue 'dir;
                }

                // FIXME: when there are multiple shortest paths, keep all to find result that doesn't break the 3-streak rule
                if let Some(recent_3) = node.dir.windows(2).last() {
                    if recent_3.iter().all(|d| d == dir) {
                        continue 'dir;
                    }
                }

                let next_distance = distances[current_pos.0][current_pos.1] + map[adj.0][adj.1];
                if distances[adj.0][adj.1] > next_distance {
                    distances[adj.0][adj.1] = next_distance;

                    let mut dir_history = node.dir.clone();
                    dir_history.push(*dir);

                    queue.push(Node {
                        position: adj.clone(),
                        dir: dir_history,
                        cost: next_distance,
                    });

                    //println!("adj(to {:?}): {:?}, distance: {next_distance}", dir,  adj);
                }
            }
        }

        //println!();
        //println!("{:?}", queue);
        distances.iter().for_each(|row| {
            row.iter().for_each(|n| {
                if *n == u32::MAX {
                    print!("INF")
                } else {
                    print!("{n:3} ")
                }
            });
            println!();
        });
    }

    distances[num_rows - 1][num_cols - 1]
}

fn in_range(pos: (i32, i32), rows: usize, cols: usize) -> bool {
    pos.0 >= 0 && pos.0 < rows as i32 && pos.1 >= 0 && pos.1 < cols as i32
}
#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Clone)]
struct Position(usize, usize);
impl Position {
    fn adj_to(&self, dir: &Direction, rows: usize, cols: usize) -> Option<Position> {
        let new_pos = match dir {
            Direction::Up => (self.0 as i32 - 1, self.1 as i32),
            Direction::Down => (self.0 as i32 + 1, self.1 as i32),
            Direction::Left => (self.0 as i32, self.1 as i32 - 1),
            Direction::Right => (self.0 as i32, self.1 as i32 + 1),
        };

        if in_range(new_pos, rows, cols) {
            Some(Position(new_pos.0 as usize, new_pos.1 as usize))
        } else {
            None
        }
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Debug, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Eq, PartialEq)]
struct Node {
    position: Position,
    dir: Vec<Direction>,
    cost: u32,
}
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
    const SIMPLE_TEST_INPUT: &str = "2413
3215
3255
3446";

    #[test]
    fn test_part1_with_test_input() {
        let result = part1(TEST_INPUT);
        assert_eq!(result, 102);
    }
}
