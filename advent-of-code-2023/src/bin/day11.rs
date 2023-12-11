use std::fs::File;
use std::io::{BufRead, BufReader};

fn day11_input() -> Vec<String> {
    let file = File::open("inputs/day11.txt").unwrap();
    BufReader::new(file).lines().map(|l| l.unwrap()).collect()
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Location(u64, u64);
impl Location {
    fn distance_to(&self, loc: Location) -> u64 {
        let row_distance = (self.0 as i64 - loc.0 as i64).abs() as u64;
        let col_distance = (self.1 as i64 - loc.1 as i64).abs() as u64;

        row_distance + col_distance
    }
}

struct Map {
    rows: u64,
    cols: u64,
    galaxies: Vec<Location>,
}
impl Map {
    fn expand(&mut self, mul: u64) {
        let empty_rows = (0..self.rows)
            .into_iter()
            .filter(|num| self.has_galaxy_in_row(*num) == false)
            .collect::<Vec<_>>();

        let empty_cols = (0..self.cols)
            .into_iter()
            .filter(|num| self.has_galaxy_in_col(*num) == false)
            .collect::<Vec<_>>();

        let new_galaxies = self
            .galaxies
            .iter()
            .map(|loc| {
                let row_expand_count =
                    Map::value_counts_before(&empty_rows, loc.0) as u64 * (mul - 1);
                let col_expand_count =
                    Map::value_counts_before(&empty_cols, loc.1) as u64 * (mul - 1);

                Location(loc.0 + row_expand_count, loc.1 + col_expand_count)
            })
            .collect::<Vec<_>>();

        self.rows += empty_rows.len() as u64;
        self.cols += empty_cols.len() as u64;
        self.galaxies = new_galaxies;
    }

    fn value_counts_before(list: &Vec<u64>, num: u64) -> usize {
        list.iter().filter(|&&n| n < num).count()
    }

    fn has_galaxy_in_row(&self, row: u64) -> bool {
        self.galaxies.iter().filter(|loc| loc.0 == row).count() != 0
    }

    fn has_galaxy_in_col(&self, col: u64) -> bool {
        self.galaxies.iter().filter(|loc| loc.1 == col).count() != 0
    }

    fn combinations(&self) -> Vec<(Location, Location)> {
        let mut combinations = Vec::new();
        for i in 0..self.galaxies.len() {
            for j in (i + 1)..self.galaxies.len() {
                combinations.push((self.galaxies[i].clone(), self.galaxies[j].clone()));
            }
        }

        combinations
    }
}

fn parse_input(input: &Vec<String>) -> Map {
    let rows = input.len() as u64;
    let cols = input[0].len() as u64;

    let galaxies = input
        .iter()
        .enumerate()
        .map(|(row_idx, line)| {
            line.chars()
                .enumerate()
                .filter(|(_col_idx, val)| *val == '#')
                .map(|(col_idx, _val)| Location(row_idx as u64, col_idx as u64))
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<Vec<_>>();

    Map {
        rows,
        cols,
        galaxies,
    }
}

fn part1(input: &Vec<String>) -> u64 {
    let mut map = parse_input(&input);
    map.expand(2);

    let all_combinations = map.combinations();
    all_combinations
        .iter()
        .map(|(a, b)| (*a).distance_to(*b))
        .collect::<Vec<_>>()
        .iter()
        .sum()
}

fn part2(input: &Vec<String>, mul: u64) -> u64 {
    let mut map = parse_input(&input);
    map.expand(mul);

    let all_combinations = map.combinations();
    all_combinations
        .iter()
        .map(|(a, b)| (*a).distance_to(*b))
        .collect::<Vec<_>>()
        .iter()
        .sum()
}
fn main() {
    let input = day11_input();
    let result = part1(&input);
    println!("result: {}", result);

    let result = part2(&input, 1000000);
    println!("result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_input() -> Vec<String> {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

        input.lines().map(|line| line.to_string()).collect()
    }

    #[test]
    fn test_expand() {
        let input = test_input();
        let mut map = parse_input(&input);
        assert_eq!(map.rows, 10);
        assert_eq!(map.cols, 10);
        assert_eq!(
            map.galaxies,
            vec![
                Location(0, 3),
                Location(1, 7),
                Location(2, 0),
                Location(4, 6),
                Location(5, 1),
                Location(6, 9),
                Location(8, 7),
                Location(9, 0),
                Location(9, 4)
            ]
        );

        map.expand(2);
        assert_eq!(map.rows, 12);
        assert_eq!(map.cols, 13);
        assert_eq!(
            map.galaxies,
            vec![
                Location(0, 4),
                Location(1, 9),
                Location(2, 0),
                Location(5, 8),
                Location(6, 1),
                Location(7, 12),
                Location(10, 9),
                Location(11, 0),
                Location(11, 5)
            ]
        );
    }

    #[test]
    fn test_part1_with_test_input() {
        let input = test_input();
        let result = part1(&input);
        assert_eq!(result, 374);
    }

    #[test]
    fn test_part1() {
        let input = day11_input();
        let result = part1(&input);
        assert_eq!(result, 9684228);
    }

    #[test]
    fn test_part2_with_test_input() {
        let input = test_input();
        let result = part2(&input, 10);
        assert_eq!(result, 1030);

        let result = part2(&input, 100);
        assert_eq!(result, 8410);
    }

    #[test]
    fn test_part2() {
        let input = day11_input();
        let result = part2(&input, 1000000);
        assert_eq!(result, 483844716556);
    }
}
