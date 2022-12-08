use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let trees = get_day8_input();

    let rows = trees.num_rows();
    let cols = trees.num_cols();

    let mut visible = 0;
    for row in 0..rows {
        for col in 0..cols {
            if trees.is_visible(row, col) {
                visible += 1;
            }
        }
    }
    println!("{}", visible);
}

fn get_day8_input() -> Trees {
    let file = File::open("inputs/day8.txt").unwrap();
    let lines = BufReader::new(file).lines();

    Trees {
        grid: lines
            .into_iter()
            .map(|l| {
                l.unwrap()
                    .chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect::<Vec<u32>>()
            })
            .collect(),
    }
}

struct Trees {
    grid: Vec<Vec<u32>>,
}
struct Range {
    start: usize,
    end: usize,
}

impl Trees {
    fn num_rows(&self) -> usize {
        self.grid.len()
    }
    fn num_cols(&self) -> usize {
        self.grid[0].len()
    }

    fn is_visible(&self, row: usize, col: usize) -> bool {
        return if row == 0 {
            true
        } else if col == 0 {
            true
        } else {
            self.is_visible_from_top(row, col)
                || self.is_visible_from_bottom(row, col)
                || self.is_visible_from_left(row, col)
                || self.is_visible_from_right(row, col)
        };
    }
    fn visible_in_range(
        &self,
        row: Range,
        col: Range,
        height: u32,
    ) -> bool {
        for r in row.start..row.end + 1 {
            for c in col.start..col.end + 1 {
                if self.grid[r][c] >= height {
                    return false;
                }
            }
        }
        return true;
    }

    fn is_visible_from_left(&self, row: usize, col: usize) -> bool {
        let height = self.grid[row][col];
        let row_range = Range { start: row, end: row };
        let col_range = Range { start: 0, end: col - 1 };
        self.visible_in_range(row_range, col_range, height)
    }
    fn is_visible_from_right(&self, row: usize, col: usize) -> bool {
        let height = self.grid[row][col];
        let row_range = Range { start: row, end: row };
        let col_range = Range { start: col + 1, end: self.num_cols() - 1 };
        self.visible_in_range(row_range, col_range, height)
    }
    fn is_visible_from_top(&self, row: usize, col: usize) -> bool {
        let height = self.grid[row][col];
        let row_range = Range { start: 0, end: row - 1 };
        let col_range = Range { start: col, end: col };
        self.visible_in_range(row_range, col_range, height)
    }
    fn is_visible_from_bottom(&self, row: usize, col: usize) -> bool {
        let height = self.grid[row][col];
        let row_range = Range { start: row + 1, end: self.num_rows() - 1 };
        let col_range = Range { start: col, end: col };
        self.visible_in_range(row_range, col_range, height)
    }
}
