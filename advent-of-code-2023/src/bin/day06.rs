use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let input = day6_input();

    let result = part1(&input);
    println!("result = {}", result);

    let result = part2(&input);
    println!("result = {}", result);
}

fn test_input() -> Vec<String> {
    let input = "Time:      7  15   30
Distance:  9  40  200";
    input.lines().map(|l| l.to_string()).collect()
}

fn day6_input() -> Vec<String> {
    let file = File::open("inputs/day06.txt").unwrap();
    let reader = BufReader::new(file);
    reader.lines().map(|l| l.unwrap()).collect()
}

#[derive(Debug, PartialEq)]
struct Race {
    time: u64,
    distance: u64,
    possibilities: HashMap<u64, u64>,
}
impl Race {
    pub fn new(time: u64) -> Self {
        Self {
            time,
            distance: 0,
            possibilities: HashMap::new(),
        }
    }
    pub fn update_possibilities(&mut self) {
        for push_time in 0..=self.time {
            let result = (self.time - push_time) * push_time;
            self.possibilities.insert(push_time, result);
        }
    }
    pub fn num_ways_to_beat(&self) -> u64 {
        self.possibilities
            .iter()
            .map(|p| if *p.1 > self.distance { 1 } else { 0 })
            .sum()
    }
}

fn get_part1_races(lines: &Vec<String>) -> Vec<Race> {
    let mut races = Vec::new();

    lines.iter().for_each(|l| {
        let numbers: Vec<_> = l
            .split(" ")
            .filter(|&c| c != "" && c.chars().all(|c| c.is_numeric()))
            .map(|c| c.parse::<u64>().unwrap())
            .collect();

        for idx in 0..numbers.len() {
            if l.starts_with("Time:") {
                races.push(Race::new(numbers[idx]));
            } else if l.starts_with("Distance:") {
                races[idx].distance = numbers[idx];
            }
        }
    });

    races
}

fn part1(input: &Vec<String>) -> u64 {
    let mut races = get_part1_races(&input);
    get_result(&mut races)
}

fn get_result(races: &mut Vec<Race>) -> u64 {
    let result: Vec<_> = races
        .iter_mut()
        .map(|r| {
            r.update_possibilities();
            r.num_ways_to_beat()
        })
        .collect();

    result.iter().fold(1, |acc, &v| acc * v)
}

fn get_part2_races(lines: &Vec<String>) -> Vec<Race> {
    let mut races = Vec::new();

    lines.iter().for_each(|l| {
        let numbers = l.split(":").collect::<Vec<_>>()[1];
        let numbers = numbers.trim().replace(" ", "");
        let number = numbers.parse::<u64>().unwrap();

        if l.starts_with("Time:") {
            races.push(Race::new(number));
        } else if l.starts_with("Distance:") {
            races.last_mut().unwrap().distance = number;
        }
    });

    races
}

fn part2(input: &Vec<String>) -> u64 {
    let mut races = get_part2_races(&input);
    // slow.. but can get the answer
    get_result(&mut races)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_parsing() {
        let input = test_input();
        let races = get_part1_races(&input);

        let expected = vec![
            Race {
                time: 7,
                distance: 9,
                possibilities: HashMap::new(),
            },
            Race {
                time: 15,
                distance: 40,
                possibilities: HashMap::new(),
            },
            Race {
                time: 30,
                distance: 200,
                possibilities: HashMap::new(),
            },
        ];
        assert_eq!(races, expected);
    }

    #[test]
    fn test_part2_parsing() {
        let input = test_input();
        let races = get_part2_races(&input);

        let expected = vec![Race {
            time: 71530,
            distance: 940200,
            possibilities: HashMap::new(),
        }];
        assert_eq!(races, expected);
    }

    #[test]
    fn test_update_possibilities() {
        let mut race = Race {
            time: 7,
            distance: 9,
            possibilities: HashMap::new(),
        };
        race.update_possibilities();

        assert_eq!(race.possibilities[&0], 0);
        assert_eq!(race.possibilities[&1], 6);
        assert_eq!(race.possibilities[&2], 10);
        assert_eq!(race.possibilities[&3], 12);
        assert_eq!(race.possibilities[&4], 12);
        assert_eq!(race.possibilities[&5], 10);
        assert_eq!(race.possibilities[&6], 6);
        assert_eq!(race.possibilities[&7], 0);
    }

    #[test]
    fn test_part1_with_test_input() {
        let input = test_input();
        let result = part1(&input);
        assert_eq!(result, 288);
    }

    #[test]
    fn test_part2_with_test_input() {
        let input = test_input();
        let result = part2(&input);
        assert_eq!(result, 71503);
    }
}
