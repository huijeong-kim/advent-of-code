use advent_of_code_2024::read_from_file;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fmt::Formatter;

struct Map {
    nodes: Vec<Vec<char>>,
}

impl std::fmt::Debug for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for line in &self.nodes {
            writeln!(f, "{}", line.into_iter().collect::<String>()).unwrap();
        }
        Ok(())
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Loc {
    row: i32,
    col: i32,
}

impl Loc {
    fn new(row: i32, col: i32) -> Self {
        Self { row, col }
    }
    fn next(&mut self, row_diff: i32, col_diff: i32) {
        self.row = self.row + row_diff;
        self.col = self.col + col_diff;
    }
}

enum LineType {
    Horizontal,
    Vertical,
    NegativeSlope,
    PositiveSlope,
}

struct Line {
    loc1: Loc,
    loc2: Loc,
}

impl Line {
    pub fn new(loc1: &Loc, loc2: &Loc) -> Self {
        let loc1: Loc = (*loc1).clone();
        let loc2: Loc = (*loc2).clone();

        Self { loc1, loc2 }
    }
    fn loc_in_order<'a>(loc1: &'a Loc, loc2: &'a Loc) -> (&'a Loc, &'a Loc, &'a Loc, &'a Loc) {
        // left, right, up, down
        let (left, right) = if loc1.col < loc2.col {
            (loc1, loc2)
        } else {
            (loc2, loc1)
        };

        // up has lower row num
        let (up, down) = if loc1.row < loc2.row {
            (loc1, loc2)
        } else {
            (loc2, loc1)
        };

        (left, right, up, down)
    }

    fn find_antinodes(&self, rows: i32, cols: i32, only_once: bool) -> HashSet<(i32, i32)> {
        let (left, right, up, down) = Line::loc_in_order(&self.loc1, &self.loc2);
        let line_type = LineType::from(left, right);

        let col_diff = right.col - left.col;
        let row_diff = down.row - up.row;
        assert!(col_diff >= 0);
        assert!(row_diff >= 0);

        let mut antinodes = HashSet::new();

        let in_range = |r: i32, c: i32| -> bool { r >= 0 && r < rows && c >= 0 && c < cols };

        let mut try_insert = |loc: &Loc| -> bool {
            return if in_range(loc.row, loc.col) {
                antinodes.insert((loc.row, loc.col));
                true
            } else {
                false
            };
        };

        let mut try_all = |row: i32, col: i32, row_diff: i32, col_diff| {
            let mut loc = Loc::new(row, col);
            while try_insert(&loc) {
                if only_once {
                    break;
                }

                loc.next(row_diff, col_diff);
            }
        };

        match line_type {
            LineType::Horizontal => {
                try_all(left.row, left.col - col_diff, 0, -col_diff);
                try_all(left.row, right.col + col_diff, 0, col_diff);
            }
            LineType::Vertical => {
                // up
                //  |
                // down
                try_all(up.row - row_diff, up.col, -row_diff, 0);
                try_all(down.row + row_diff, up.col, row_diff, 0);
            }
            LineType::NegativeSlope => {
                // up
                //          down
                try_all(up.row - row_diff, up.col - col_diff, -row_diff, -col_diff);
                try_all(down.row + row_diff, down.col + col_diff, row_diff, col_diff);
            }
            LineType::PositiveSlope => {
                //          up
                // down
                try_all(up.row - row_diff, up.col + col_diff, -row_diff, col_diff);
                try_all(
                    down.row + row_diff,
                    down.col - col_diff,
                    row_diff,
                    -col_diff,
                );
            }
        }
        antinodes
    }
}

impl LineType {
    pub fn from(left: &Loc, right: &Loc) -> Self {
        assert!(left.col <= right.col);
        return if left.row == right.row {
            LineType::Horizontal
        } else if left.col == right.col {
            LineType::Vertical
        } else if left.row < right.row {
            LineType::NegativeSlope
        } else if left.row > right.row {
            LineType::PositiveSlope
        } else {
            panic!("should not reach here")
        };
    }
}

impl Map {
    pub fn new(input: &str) -> Self {
        Map {
            nodes: input.lines().map(|l| l.chars().collect()).collect(),
        }
    }

    fn group_nodes(&self) -> HashMap<char, Vec<Loc>> {
        let mut groups: HashMap<char, Vec<Loc>> = HashMap::new();

        for row in 0..self.nodes.len() {
            for (col, ch) in self.nodes[row].iter().enumerate() {
                if *ch != '.' {
                    let loc = Loc {
                        row: row as i32,
                        col: col as i32,
                    };
                    groups.entry(*ch).or_insert_with(Vec::new).push(loc);
                }
            }
        }
        groups
    }

    pub fn update_antinodes(&mut self, only_once: bool) {
        let rows = self.nodes.len() as i32;
        let cols = self.nodes[0].len() as i32;

        let groups = self.group_nodes();
        for (_key, group) in groups.iter() {
            for perm in group.iter().permutations(2).unique() {
                let line = Line::new(perm[0], perm[1]);
                let to_be_updated = line.find_antinodes(rows, cols, only_once);
                for node in to_be_updated {
                    self.nodes[node.0 as usize][node.1 as usize] = '#';
                }
            }
        }
    }

    pub fn antinodes(&self, count_all: bool) -> i32 {
        self.nodes
            .iter()
            .map(|row| {
                row.iter()
                    .filter(|&&c| if count_all { c != '.' } else { c == '#' })
                    .count()
            })
            .sum::<usize>() as i32
    }
}

fn main() -> Result<(), std::io::Error> {
    let input = read_from_file("inputs/day08.txt")?;
    println!("part1={}", part1(&input));
    println!("part2={}", part2(&input));
    Ok(())
}

fn part1(input: &str) -> i32 {
    let mut map = Map::new(input);
    map.update_antinodes(true);
    map.antinodes(false)
}

fn part2(input: &str) -> i32 {
    let mut map = Map::new(input);
    map.update_antinodes(false);
    map.antinodes(true)
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 14);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 34);
    }
}
