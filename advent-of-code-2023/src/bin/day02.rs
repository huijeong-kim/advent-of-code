use std::fs::File;
use std::io::{BufRead, BufReader};

const MAX_RED_CUBES: u32 = 12;
const MAX_GREEN_CUBES: u32 = 13;
const MAX_BLUE_CUBES: u32 = 14;

// input:  2, blue
fn to_cube_counts(count_str: &str) -> (&str, u32) {
    let v: Vec<_> = count_str.split(' ').filter(|c| *c != "").collect();
    let count: u32 = v[0].parse().unwrap();
    let color = v[1];

    (color, count)
}

fn is_valid(color: &str, count: u32) -> bool {
    return if color == "blue" && count <= MAX_BLUE_CUBES {
        true
    } else if color == "green" && count <= MAX_GREEN_CUBES {
        true
    } else if color == "red" && count <= MAX_RED_CUBES {
        true
    } else {
        false
    };
}

#[derive(Default)]
struct Cubes {
    blue: u32,
    red: u32,
    green: u32,
}
impl Cubes {
    pub fn update(&mut self, color: &str, count: u32) {
        match color {
            "blue" if self.blue < count => self.blue = count,
            "red" if self.red < count => self.red = count,
            "green" if self.green < count => self.green = count,
            _ => {} // do nothing
        }
    }
    pub fn get_result(&self) -> u32 {
        self.blue * self.red * self.green
    }
}

fn main() {
    let file = File::open("inputs/day02.txt").unwrap();
    let lines = BufReader::new(file).lines();
    let lines: Vec<String> = lines.into_iter().map(|line| line.unwrap()).collect();

    // Part 1
    let result = part1(&lines);
    println!("result: {}", result);

    // Part 2
    let result = part2(&lines);
    println!("result: {}", result);
}

fn part1(lines: &Vec<String>) -> u32 {
    let result: Vec<u32> = lines
        .iter()
        .map(|l| {
            let split: Vec<&str> = l.split(':').collect();
            let game_id = split[0].split(' ').last().unwrap();
            let trials: Vec<_> = split[1].split(';').collect();

            let result: Vec<u32> = trials
                .iter()
                .map(|trial| {
                    trial
                        .split(',')
                        .collect::<Vec<_>>()
                        .iter()
                        .map(|count_str| {
                            let (color, count) = to_cube_counts(count_str);
                            if is_valid(color, count) {
                                0
                            } else {
                                1
                            }
                        })
                        .sum()
                })
                .collect();
            let result: u32 = result.iter().sum();
            // result is number of impossible combination

            if result == 0 {
                game_id.parse::<u32>().unwrap()
            } else {
                0
            }
        })
        .collect();

    result.iter().sum()
}

fn part2(lines: &Vec<String>) -> u32 {
    let result: Vec<u32> = lines
        .iter()
        .map(|l| {
            let split: Vec<&str> = l.split(':').collect();
            let trials: Vec<_> = split[1].split(';').collect();

            let mut cube_counts = Cubes::default();
            trials.iter().for_each(|t| {
                let counts: Vec<&str> = t.split(',').collect();
                counts.iter().for_each(|c| {
                    let (color, count) = to_cube_counts(*c);
                    cube_counts.update(color, count);
                });
            });

            cube_counts.get_result()
        })
        .collect();
    result.iter().sum()
}
