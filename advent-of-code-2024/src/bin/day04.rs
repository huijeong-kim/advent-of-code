use advent_of_code_2024::read_from_file;

struct Map {
    lines: Vec<String>,
    rows: i32,
    cols: i32,
}

const HORIZONTAL: i32 = 0;
const VERTICAL: i32 = 1;
const BACKWARD: i32 = 2;
const DIAGONAL_RD: i32 = 3;
const DIAGONAL_RU: i32 = 4;
const DIAGONAL_LD: i32 = 5;
const DIAGONAL_LU: i32 = 6;
const UPWARD: i32 = 7;
const NUM_DIR: i32 = 8;

const ROW: usize = 0;
const COL: usize = 1;

static CHARS: &'static [char] = &['X', 'M', 'A', 'S'];
static DIRECTIONS: &'static [[&[i32]; 2]; NUM_DIR as usize] = &[
    [&[0, 0, 0, 0], &[0, 1, 2, 3]],       // HORIZONTAL
    [&[0, 1, 2, 3], &[0, 0, 0, 0]],       // VERTICAL
    [&[0, 0, 0, 0], &[0, -1, -2, -3]],    // BACKWARD
    [&[0, 1, 2, 3], &[0, 1, 2, 3]],       // DIAGONAL_RD
    [&[0, -1, -2, -3], &[0, 1, 2, 3]],    // DIAGONAL_RU
    [&[0, 1, 2, 3], &[0, -1, -2, -3]],    // DIAGONAL_LD
    [&[0, -1, -2, -3], &[0, -1, -2, -3]], // DIAGONAL_LU
    [&[0, -1, -2, -3], &[0, 0, 0, 0]],    // UPWARD
];

impl Map {
    fn new(input: &str) -> Map {
        let lines = input.lines();
        let lines = lines.map(|line| line.to_string()).collect::<Vec<String>>();
        let rows = lines.len() as i32;
        let cols = lines[0].len() as i32;
        assert!(lines.len() > 1);
        Map { lines, rows, cols }
    }
    fn in_range(&self, r: i32, c: i32) -> bool {
        r >= 0 && r < self.rows && c >= 0 && c < self.cols
    }
    fn is_char(&self, r: i32, c: i32, ch: char) -> bool {
        self.in_range(r, c) && self.lines[r as usize].chars().nth(c as usize).unwrap() == ch
    }

    fn is_xmas(&self, r: i32, c: i32, dir: i32) -> bool {
        for i in 0..4 { // For chars in XMAS
            if !self.is_char(
                r + DIRECTIONS[dir as usize][ROW][i],
                c + DIRECTIONS[dir as usize][COL][i],
                CHARS[i],
            ) {
                return false;
            }
        }
        true
    }
    fn is_mas(&self, r: i32, c: i32, dir: i32) -> bool {
        for i in 0..3 { // For chars in  MAS
            if !self.is_char(
                r + DIRECTIONS[dir as usize][ROW][i],
                c + DIRECTIONS[dir as usize][COL][i],
                CHARS[i + 1], // to skip 'X'
            ) {
                return false;
            }
        }
        true
    }

    fn find_xmas(&self) -> i32 {
        let mut count = 0;

        for r in 0..self.rows {
            for c in 0..self.cols {
                if self.lines[r as usize].chars().nth(c as usize).unwrap() == 'X' {
                    for i in 0..NUM_DIR {
                        if self.is_xmas(r, c, i) {
                            count += 1;
                        }
                    }
                }
            }
        }
        count
    }

    fn is_x_mas_box(&self, r: i32, c: i32) -> bool {
        let mut mas_count = 0;

        if self.in_range(r, c) && self.is_mas(r, c, DIAGONAL_RD) {
            mas_count += 1;
        }
        if self.in_range(r, c + 2) && self.is_mas(r, c + 2, DIAGONAL_LD) {
            mas_count += 1;
        }
        if self.in_range(r + 2, c) && self.is_mas(r + 2, c, DIAGONAL_RU) {
            mas_count += 1;
        }
        if self.in_range(r + 2, c + 2) && self.is_mas(r + 2, c + 2, DIAGONAL_LU) {
            mas_count += 1;
        }
        mas_count == 2
    }
    fn find_x_mas(&self) -> i32 {
        let mut count = 0;
        for r in 0..self.rows {
            for c in 0..self.cols {
                if self.is_x_mas_box(r, c) {
                    count += 1;
                }
            }
        }
        count
    }
}
fn part1(input: &str) -> i32 {
    let map = Map::new(input);
    map.find_xmas()
}

fn part2(input: &str) -> i32 {
    let map = Map::new(input);
    map.find_x_mas()
}

fn main() -> Result<(), std::io::Error> {
    let input = read_from_file("inputs/day04.txt")?;
    println!("part1 = {}", part1(&input));
    println!("part2 = {}", part2(&input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 18);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 9);
    }
}
