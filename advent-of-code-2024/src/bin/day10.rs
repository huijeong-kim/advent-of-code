use advent_of_code_2024::read_from_file;
use std::collections::{HashMap, HashSet};

fn main() -> Result<(), std::io::Error> {
    let input = read_from_file("inputs/day10.txt")?;

    let mut map = Map::new(&input);
    map.find();

    let part1: i32 = map
        .dest
        .iter()
        .map(|(_start, dest)| dest.len() as i32)
        .sum();
    let part2: i32 = map.paths.iter().map(|(_loc, val)| val).sum();
    println!("part1={part1}");
    println!("part2={part2}");
    Ok(())
}

fn part1(input: &str) -> i32 {
    let mut map = Map::new(input);
    map.find();
    map.dest
        .iter()
        .map(|(_start, dest)| dest.len() as i32)
        .sum()
}

fn part2(input: &str) -> i32 {
    let mut map = Map::new(input);
    map.find();
    map.paths.iter().map(|(_loc, val)| val).sum()
}

#[derive(Clone, Eq, Hash, PartialEq)]
struct Loc {
    row: i32,
    col: i32,
}

impl Loc {
    fn right(self) -> Loc {
        Loc {
            row: self.row,
            col: self.col + 1,
        }
    }

    fn left(self) -> Loc {
        Loc {
            row: self.row,
            col: self.col - 1,
        }
    }

    fn up(self) -> Loc {
        Loc {
            row: self.row - 1,
            col: self.col,
        }
    }

    fn down(self) -> Loc {
        Loc {
            row: self.row + 1,
            col: self.col,
        }
    }
}

struct Array2D<T> {
    vals: Vec<Vec<T>>,
}

impl<T: Copy> Array2D<T> {
    fn as_mut(&mut self, loc: &Loc) -> &mut T {
        &mut self.vals[loc.row as usize][loc.col as usize]
    }
    fn get(&self, loc: &Loc) -> T {
        self.vals[loc.row as usize][loc.col as usize]
    }
}

struct Map {
    heights: Array2D<i32>,
    rows: usize,
    cols: usize,
    dest: HashMap<Loc, HashSet<Loc>>, // from, to, for scores
    paths: HashMap<Loc, i32>,         // from, # paths, for ratings
}

impl Map {
    fn new(input: &str) -> Self {
        let heights: Vec<Vec<i32>> = input
            .lines()
            .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as i32).collect())
            .collect();
        let rows = heights.len();
        let cols = heights[0].len();
        Self {
            heights: Array2D { vals: heights },
            rows,
            cols,
            dest: HashMap::new(),
            paths: HashMap::new(),
        }
    }

    fn find(&mut self) {
        let mut visited = Array2D { vals: vec![vec![false; self.cols]; self.rows] };
        for row in 0..self.rows {
            for col in 0..self.cols {
                let loc = Loc {
                    row: row as i32,
                    col: col as i32,
                };
                self.traverse(loc.clone(), 0, &loc, &mut visited);
            }
        }
    }

    fn in_range(&self, loc: &Loc) -> bool {
        loc.row >= 0 && loc.row < self.rows as i32 && loc.col >= 0 && loc.col < self.cols as i32
    }
    fn traverse(&mut self, loc: Loc, val: i32, start: &Loc, visited: &mut Array2D<bool>) {
        if !self.in_range(&loc) {
            return;
        }

        if self.heights.get(&loc) != val {
            return;
        }

        if visited.get(&loc) {
            return;
        }

        if val == 9 {
            self.dest
                .entry(start.clone())
                .or_insert(HashSet::new())
                .insert(loc.clone());

            let prev = self.paths.get(start).unwrap_or(&0);
            self.paths.insert(start.clone(), prev + 1);
            return;
        }

        *visited.as_mut(&loc) = true;

        // right
        self.traverse(loc.clone().right(), val + 1, start, visited);
        // left
        self.traverse(loc.clone().left(), val + 1, start, visited);
        // up
        self.traverse(loc.clone().up(), val + 1, start, visited);
        // down
        self.traverse(loc.clone().down(), val + 1, start, visited);

        *visited.as_mut(&loc) = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 36);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 81);
    }
}
