use advent_of_code_2024::read_from_file;

const FREE: i32 = -1;

struct Blocks {
    ids: Vec<i32>,
}

impl From<&Blocks> for String {
    fn from(value: &Blocks) -> Self {
        value
            .ids
            .iter()
            .map(|&val| {
                if val == FREE {
                    '.'
                } else {
                    std::char::from_digit(val as u32, 10).unwrap()
                }
            })
            .collect()
    }
}

impl Blocks {
    pub fn new(input: &str) -> Self {
        let mut id = 0;
        let vals = input
            .chars()
            .enumerate()
            .flat_map(|(idx, ch)| {
                let num_blocks: u32 = ch.to_digit(10).unwrap();
                if idx % 2 == 0 {
                    let cur_id = id;
                    id += 1;
                    std::iter::repeat(cur_id).take(num_blocks as usize)
                } else {
                    std::iter::repeat(FREE).take(num_blocks as usize)
                }
            })
            .collect();

        Self { ids: vals }
    }

    fn move_block(&mut self, from: usize, to: usize) {
        assert!(self.is_valid_idx(from) && self.is_valid_idx(to));
        self.ids[to] = self.ids[from];
        self.ids[from] = FREE;
    }

    fn is_valid_idx(&self, idx: usize) -> bool {
        idx < self.ids.len()
    }

    fn find_next_free_block(&self, idx: usize) -> Option<usize> {
        assert!(self.is_valid_idx(idx));
        // from the idx to the right direction, find the first free block
        let mut idx = idx;
        while self.is_valid_idx(idx) && self.ids[idx] != FREE {
            idx += 1;
        }

        return if self.is_valid_idx(idx) {
            Some(idx)
        } else {
            None
        };
    }

    fn find_next_valid_block(&self, idx: usize) -> Option<usize> {
        // From the idx from right to left, find the first valid block
        let mut idx = idx;
        while self.is_valid_idx(idx) && self.ids[idx] == FREE {
            if idx == 0 { return None; }
            idx -= 1;
        }

        return if self.is_valid_idx(idx) {
            Some(idx)
        } else {
            None
        };
    }


    fn zip_blocks(&mut self) {
        let mut write_idx = 0;
        let mut read_idx = self.ids.len() - 1;

        while write_idx < read_idx && write_idx < self.ids.len() {
            if let Some(idx) = self.find_next_free_block(write_idx) {
                write_idx = idx;
            } else {
                break;
            }

            if let Some(idx) = self.find_next_valid_block(read_idx) {
                read_idx = idx;
            } else {
                break;
            }

            if write_idx >= read_idx {
                break;
            }

            self.move_block(read_idx, write_idx);
            // println!("{}", String::from(self.deref()));
        }
    }

    fn find_num_blocks(&self, idx: usize, to_left: bool) -> usize {
        assert!(self.is_valid_idx(idx));

        let mut idx = idx;
        let target = self.ids[idx];

        let mut num_blocks= 0;
        while self.is_valid_idx(idx) && self.ids[idx] == target {
            num_blocks += 1;

            if to_left {
                if idx == 0 { break; }
                idx -= 1;
            } else {
                idx += 1;
            }
        }

        num_blocks
    }

    fn zip_blocks_by_id(&mut self) {
        let mut write_idx = 0;
        let mut read_idx = self.ids.len() - 1;

        while write_idx < read_idx && write_idx < self.ids.len() {
            if let Some(idx) = self.find_next_valid_block(read_idx) {
                read_idx = idx;
            } else {
                break;
            }
            let num_blocks = self.find_num_blocks(read_idx, true);

            while write_idx < self.ids.len() {
                if let Some(idx) = self.find_next_free_block(write_idx) {
                    write_idx = idx;
                } else {
                    write_idx = self.ids.len();
                    break;
                }
                let num_free_blocks = self.find_num_blocks(write_idx, false);

                if num_free_blocks < num_blocks {
                    write_idx += num_free_blocks;
                } else {
                    // found !
                    break;
                }
            }

            if self.is_valid_idx(write_idx) && write_idx < read_idx {
                for _ in 0..num_blocks {
                    self.move_block(read_idx, write_idx);
                    read_idx -= 1;
                    write_idx += 1;
                }

                // println!("{}", String::from(self.deref()));
                // println!("write_idx={write_idx}, read_idx={read_idx}");
            } else {
                if read_idx < num_blocks {
                    break;
                }
                read_idx -= num_blocks;
                write_idx = 0;
            }
        }
    }

    fn checksum(&self) -> i64 {
        self.ids
            .iter()
            .enumerate()
            .map(|(idx, &val)| {
                if val != FREE {
                    val as i64 * idx as i64
                } else {
                    0
                }
            })
            .sum()
    }
}

fn part1(input: &str) -> i64 {
    let mut blockmap = Blocks::new(input);
    blockmap.zip_blocks();
    blockmap.checksum()
}

fn part2(input: &str) -> i64 {
    let mut blockmap = Blocks::new(input);
    blockmap.zip_blocks_by_id();
    blockmap.checksum()
}

fn main() -> Result<(), std::io::Error> {
    let input = read_from_file("inputs/day09.txt")?;
    println!("part1={}", part1(&input));
    println!("part2={}", part2(&input)); // 7389006017011 too high
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2333133121414131402";

    #[test]
    fn test_utils() {
        let blockmap = Blocks::new(INPUT);
        let result: String = (&blockmap).into();

        assert_eq!(
            result.as_str(),
            "00...111...2...333.44.5555.6666.777.888899"
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 1928);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 2858);
    }
}
