use std::collections::HashSet;

use advent_of_code_2025::read_from_file;

fn main() -> Result<(), std::io::Error> {
    let input = read_from_file("inputs/day04.txt")?;
    let mut map = Map::from(&input);
    println!("part1={}", part1(&map));
    println!("part2={}", part2(&mut map));
    Ok(())
}

#[derive(Debug)]
struct Map {
    map: Vec<Vec<char>>,
    rows: i32,
    cols: i32,
}

impl Map {
    fn from(input: &str) -> Self {
        let mut map: Vec<Vec<char>> = Vec::new();
        input.lines().for_each(|line| {
            map.push(line.chars().collect())
        });

        let rows = map.len() as i32;
        let cols = map[0].len() as i32;

        Map {
            map,
            rows,
            cols,
        }
    }

    fn adjacent_rolls(&self, r: i32, c: i32) -> i32 {
        let mut count = 0;
        let adjacents = [
            (r-1, c-1),
            (r-1, c),
            (r-1, c+1),
            (r, c-1),
            (r, c+1),
            (r+1, c-1),
            (r+1, c),
            (r+1, c+1),
        ];

        for (r, c) in adjacents {
            if self.is_valid(r, c) && self.map[r as usize][c as usize] == '@' {                
                count += 1;
            }
        }

        count
    }

    fn is_valid(&self, r: i32, c: i32) -> bool {
        r >= 0 && r < self.rows && c >= 0 && c < self.cols
    }

    fn is_accessible(&self, r:i32, c: i32) -> bool  {
        if !self.is_valid(r, c) {
            return false;
        }
        self.map[r as usize][c as usize] == '@' && self.adjacent_rolls(r, c) < 4
    }
}

fn part1(map: &Map) -> i64 {
    map.map.iter().enumerate().map(|(r, row)| {
        row.iter().enumerate().filter(|(c, _cell)| {
            map.is_accessible(r as i32, *c as i32)
        }).count()
    }).sum::<usize>() as i64
}

fn part2(map: &mut Map) -> i64 {
    let mut ret = 0;
    loop {
        let mut accessible = HashSet::new();
        for r in 0..map.rows {
            for c in 0 ..map.cols {
                if map.is_accessible(r, c) {
                    accessible.insert((r, c));
                }
            }
        }
        if accessible.len() == 0 {
            break;
        }

        ret += accessible.len();

        accessible.iter().for_each(|(r, c)| {
            map.map[*r as usize][*c as usize] = 'x';
        });
    }

    ret as i64
}
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn test_part1() {
        let map = Map::from(INPUT);
        assert_eq!(part1(&map), 13);
    }

    #[test]
    fn test_part2() {
        let mut map = Map::from(INPUT);
        assert_eq!(part2(&mut map), 43);
    }
}