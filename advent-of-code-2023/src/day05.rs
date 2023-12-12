use std::fmt::{Debug, Formatter};

pub fn solution(input: String) {
    let (seeds, maps) = get(&input);

    let result = part1(&seeds, &maps);
    println!("part1: {}", result);

    let result = part2(&seeds, &maps);
    println!("part2: {:?}", result);
}

#[derive(Clone, PartialOrd, Ord, PartialEq, Eq)]
struct Range {
    start: u128,
    end: u128,
}
impl Debug for Range {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} <= value <= {}", self.start, self.end)
    }
}
impl Range {
    pub fn new(start: u128, end: u128) -> Self {
        Self { start, end }
    }
    pub fn in_range(&self, number: u128) -> bool {
        self.start <= number && number <= self.end
    }
}

#[derive(Debug, Ord, PartialOrd, PartialEq, Eq)]
struct RangeMap {
    src_range: Range,
    dest_start: u128,
}
impl RangeMap {
    pub fn map(&self, src: u128) -> Option<u128> {
        if self.src_range.in_range(src) {
            Some(self.dest_start + (src - self.src_range.start))
        } else {
            None
        }
    }
}

#[derive(Debug, Default, PartialEq)]
struct Map {
    range_maps: Vec<RangeMap>, // src, dest, len
}

impl Map {
    pub fn map(&self, seed: u128) -> u128 {
        for range_map in &self.range_maps {
            if let Some(result) = range_map.map(seed) {
                return result;
            }
        }

        panic!("should not happen");
    }

    pub fn map_range(&self, seed: &Range) -> Vec<Range> {
        let split = self.split_range(seed);

        split
            .iter()
            .map(|src| Range::new(self.map(src.start), self.map(src.end)))
            .collect()
    }

    fn split_range(&self, seed: &Range) -> Vec<Range> {
        let mut ranges = Vec::new();

        let mut next_find = seed.start;
        // range_maps is sorted already..
        for range_map in &self.range_maps {
            if seed.in_range(next_find) == false {
                break;
            }

            if range_map.src_range.in_range(next_find) {
                let current_end = if range_map.src_range.end > seed.end {
                    seed.end
                } else {
                    range_map.src_range.end
                };

                ranges.push(Range::new(next_find, current_end));
                next_find = current_end + 1;
            }
        }

        ranges
    }
}

fn get(input: &str) -> (Vec<u128>, Vec<Map>) {
    let (seeds, mut maps) = get_input(input);
    fill_up_maps(&mut maps);

    (seeds, maps)
}

fn get_input(input: &str) -> (Vec<u128>, Vec<Map>) {
    let mut seeds = Vec::new();
    let mut maps = Vec::new();

    input.lines().for_each(|line| {
        if line.starts_with("seeds:") {
            let numbers = line.split(":").collect::<Vec<_>>()[1];
            seeds = numbers
                .split(" ")
                .filter(|n| *n != " " && *n != "")
                .map(|n| n.parse::<u128>().unwrap())
                .collect();
        } else if line.contains("map:") {
            maps.push(Map {
                range_maps: Vec::new(),
            });
        } else if line.is_empty() {
            // skip
        } else {
            // numbers
            let numbers: Vec<u128> = line
                .split(" ")
                .map(|n| n.parse::<u128>().unwrap())
                .collect();
            assert_eq!(numbers.len(), 3);

            let dest_range_start = numbers[0];
            let src_range_start = numbers[1];
            let num_values = numbers[2];

            let map = maps.last_mut().unwrap();
            map.range_maps.push(RangeMap {
                src_range: Range::new(src_range_start, src_range_start + num_values - 1),
                dest_start: dest_range_start,
            });
        }
    });

    (seeds, maps)
}

fn fill_up_maps(maps: &mut Vec<Map>) {
    // fill up the remaining ranges
    maps.iter_mut().for_each(|m| {
        m.range_maps.sort();

        let mut last_start = 0;
        let mut new_ranges = Vec::new();
        for range in &mut m.range_maps {
            if last_start < range.src_range.start {
                new_ranges.push(RangeMap {
                    src_range: Range::new(last_start, range.src_range.start - 1),
                    dest_start: last_start,
                });

                last_start = range.src_range.end + 1;
            } else if last_start == range.src_range.start {
                last_start = range.src_range.end + 1;
            }
        }

        new_ranges.push(RangeMap {
            src_range: Range {
                start: last_start,
                end: u128::MAX - 1,
            },
            dest_start: last_start,
        });
        m.range_maps.append(&mut new_ranges);

        m.range_maps.sort();
    });
}

fn find_location(seed: u128, maps: &Vec<Map>) -> u128 {
    maps.iter().fold(seed, |acc, m| m.map(acc))
}

fn part1(seeds: &Vec<u128>, maps: &Vec<Map>) -> u128 {
    let locations: Vec<_> = seeds.iter().map(|n| find_location(*n, &maps)).collect();
    *locations.iter().min().unwrap()
}

fn get_seed_range(seeds: &Vec<u128>) -> Vec<Range> {
    let mut range_seeds = Vec::new();
    for (idx, value) in seeds.iter().enumerate() {
        if idx % 2 == 0 {
            range_seeds.push(Range {
                start: *value,
                end: 0u128, // to be updated
            })
        } else {
            let last = range_seeds.last_mut().unwrap();
            last.end = last.start + *value - 1;
        }
    }

    range_seeds
}

// timeout version
#[allow(dead_code)]
fn part2_timeout(seeds: &Vec<u128>, maps: &Vec<Map>) -> u128 {
    let range_seeds = get_seed_range(seeds);

    *range_seeds
        .iter()
        .map(|seed| {
            let mut locations = Vec::new();
            for val in seed.start..seed.start + seed.end {
                locations.push(find_location(val, &maps));
            }
            println!("locations: {:?}", locations);
            *locations.iter().min().unwrap()
        })
        .collect::<Vec<_>>()
        .iter()
        .min()
        .unwrap()
}

fn find_location_in_range(seed: &Range, maps: &Vec<Map>) -> u128 {
    let mut input = vec![(*seed).clone()];

    for map in maps {
        let result = input
            .iter()
            .map(|i| map.map_range(i))
            .collect::<Vec<Vec<_>>>()
            .into_iter()
            .flatten()
            .collect();

        input = result;
    }

    input.iter().map(|n| n.start).min().unwrap()
}

fn part2(seeds: &Vec<u128>, maps: &Vec<Map>) -> u128 {
    let range_seeds = get_seed_range(&seeds);
    range_seeds
        .iter()
        .map(|seed| find_location_in_range(seed, maps))
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn test_part1_with_test_input() {
        let (seeds, mut maps) = get_input(TEST_INPUT);
        assert_eq!(seeds, vec![79, 14, 55, 13]);
        assert_eq!(maps.len(), 7);
        assert_eq!(
            maps[0],
            Map {
                range_maps: vec![
                    RangeMap {
                        src_range: Range::new(98, 99),
                        dest_start: 50,
                    },
                    RangeMap {
                        src_range: Range::new(50, 97),
                        dest_start: 52,
                    },
                ],
            }
        );

        fill_up_maps(&mut maps);
        assert_eq!(
            maps[0],
            Map {
                range_maps: vec![
                    RangeMap {
                        src_range: Range::new(0, 49),
                        dest_start: 0,
                    },
                    RangeMap {
                        src_range: Range::new(50, 97),
                        dest_start: 52,
                    },
                    RangeMap {
                        src_range: Range::new(98, 99),
                        dest_start: 50,
                    },
                    RangeMap {
                        src_range: Range::new(100, u128::MAX - 1),
                        dest_start: 100,
                    }
                ]
            }
        );

        let result = part1(&seeds, &maps);
        assert_eq!(result, 35);
    }

    #[test]
    fn test_part1() {
        let input = crate::read_from_file("inputs/day05.txt");
        let (seeds, maps) = get(&input);
        let result = part1(&seeds, &maps);
        assert_eq!(result, 165788812);
    }

    #[test]
    fn test_part2_with_test_input() {
        let (seeds, maps) = get(TEST_INPUT);
        let result = part2(&seeds, &maps);
        assert_eq!(result, 46);
    }

    #[test]
    fn test_part2() {
        let input = crate::read_from_file("inputs/day05.txt");
        let (seeds, maps) = get(&input);
        let result = part2(&seeds, &maps);
        assert_eq!(result, 1928058);
    }

    #[test]
    fn test_range_map() {
        let range_map = RangeMap {
            src_range: Range::new(10, 200),
            dest_start: 30,
        };

        assert_eq!(range_map.map(15), Some(35));
        assert_eq!(range_map.map(1), None);
    }

    #[test]
    fn test_range_maps() {
        let mut map = Map::default();
        map.range_maps.push(RangeMap {
            src_range: Range::new(0, 20),
            dest_start: 10,
        });
        map.range_maps.push(RangeMap {
            src_range: Range::new(20, 100),
            dest_start: 200,
        });
        map.range_maps.push(RangeMap {
            src_range: Range::new(100, u128::MAX),
            dest_start: 100,
        });

        assert_eq!(map.map(10), 20);
        assert_eq!(map.map(30), 210);
        assert_eq!(map.map(400), 400);
    }

    #[test]
    fn test_get_seed_range() {
        let seeds = vec![79, 14, 55, 13];
        let ranges = get_seed_range(&seeds);

        let expected = vec![Range::new(79, 79 + 14 - 1), Range::new(55, 55 + 13 - 1)];

        assert_eq!(ranges, expected);
    }

    #[test]
    fn test_range_split() {
        let mut map = Map::default();
        map.range_maps.push(RangeMap {
            src_range: Range::new(0, 19),
            dest_start: 10,
        });
        map.range_maps.push(RangeMap {
            src_range: Range::new(20, 99),
            dest_start: 200,
        });
        map.range_maps.push(RangeMap {
            src_range: Range::new(100, u128::MAX),
            dest_start: 100,
        });

        let src_range = Range::new(10, 300);
        let expected = vec![Range::new(10, 19), Range::new(20, 99), Range::new(100, 300)];
        assert_eq!(map.split_range(&src_range), expected);
    }

    #[test]
    fn test_find_location_in_range() {
        let seed = Range::new(10, 600);
        let map = Map {
            range_maps: vec![
                RangeMap {
                    src_range: Range::new(0, 29),
                    dest_start: 0,
                },
                RangeMap {
                    src_range: Range::new(30, 59),
                    dest_start: 100,
                },
                RangeMap {
                    src_range: Range::new(60, 499),
                    dest_start: 20,
                },
                RangeMap {
                    src_range: Range::new(500, u128::MAX - 1),
                    dest_start: 500,
                },
            ],
        };

        let expected = vec![
            Range::new(10, 29),
            Range::new(30, 59),
            Range::new(60, 499),
            Range::new(500, 600),
        ];
        assert_eq!(map.split_range(&seed), expected);

        let expected = vec![
            Range::new(10, 29),
            Range::new(100, 100 + (59 - 30)),
            Range::new(20, 20 + (499 - 60)),
            Range::new(500, 600),
        ];
        assert_eq!(map.map_range(&seed), expected);
    }
}
