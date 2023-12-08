use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn day8_input() -> Vec<String> {
    let file = File::open("inputs/day8.txt").unwrap();
    let buf_reader = BufReader::new(file);
    buf_reader.lines().map(|line| line.unwrap()).collect()
}

fn part1(instruction: &String, maps: &HashMap<String, Map>) -> u64 {
    let mut current = maps.get("AAA").unwrap();
    let mut count = 0;

    while instruction.chars().all(|c| {
        count += 1;

        let next = current.next(c);
        if next == "ZZZ" {
            false
        } else {
            current = maps.get(next).unwrap();
            true
        }
    }) {}

    return count;
}

#[derive(Debug)]
struct Path {
    current_node: String,
    arrived: bool,
}
impl Path {
    pub fn update(&mut self, node: String) {
        if node.ends_with('Z') {
            self.arrived = true;
        } else {
            self.arrived = false;
        }
        self.current_node = node;
    }
}

// brute-force way : timeout
#[allow(dead_code)]
fn part2_timeout(instruction: &String, maps: &HashMap<String, Map>) -> u64 {
    let mut paths = maps
        .iter()
        .filter(|(name, _)| name.ends_with('A'))
        .map(|(name, _)| Path {
            current_node: name.to_string(),
            arrived: false,
        })
        .collect::<Vec<_>>();

    let mut count = 0;
    let mut found = false;

    while instruction.chars().all(|c| {
        count += 1;

        paths.iter_mut().for_each(|node| {
            let current = maps.get(&*node.current_node).unwrap();
            let next = maps.get(current.next(c)).unwrap();

            node.update(next.destination.to_string());
        });

        if paths.iter().filter(|path| path.arrived == false).count() == 0 {
            found = true;
        }

        !found
    }) {}

    return count;
}

fn part2(instruction: &String, maps: &HashMap<String, Map>) -> u64 {
    let mut paths = maps
        .iter()
        .filter(|(name, _)| name.ends_with('A'))
        .map(|(name, _)| Path {
            current_node: name.to_string(),
            arrived: false,
        })
        .collect::<Vec<_>>();

    let path_results = paths
        .iter_mut()
        .map(|node| {
            let mut count = 0;

            while instruction.chars().all(|c| {
                count += 1;

                let current = maps.get(&*node.current_node).unwrap();
                let next = maps.get(current.next(c)).unwrap();
                node.update(next.destination.to_string());

                !node.arrived
            }) {}

            count
        })
        .collect::<Vec<_>>();

    path_results.iter().cloned().fold(1, num::integer::lcm)
}

fn main() {
    let input = day8_input();
    let (instruction, maps) = parse_input(&input);

    let result = part1(&instruction, &maps);
    println!("result: {:?}", result);

    let result = part2(&instruction, &maps);
    println!("result: {:?}", result);
}

#[derive(PartialEq, Debug)]
struct Map {
    destination: String,
    left: String,
    right: String,
}
impl Map {
    pub fn next(&self, direction: char) -> &str {
        if direction == 'L' {
            self.left.as_str()
        } else if direction == 'R' {
            self.right.as_str()
        } else {
            panic!("can't reach here")
        }
    }
}

fn parse_input(input: &Vec<String>) -> (String, HashMap<String, Map>) {
    let instruction = input[0].clone();

    let maps = input
        .iter()
        .map(|line| {
            let split = line.split("=").collect::<Vec<_>>();
            if split.len() == 2 {
                let destination = split[0].trim_matches(' ');

                let map = split[1].split(',').collect::<Vec<_>>();
                assert_eq!(map.len(), 2);

                let left = map[0].trim_matches(' ').trim_matches('(');
                let right = map[1].trim_matches(')').trim_matches(' ');

                Some((
                    destination.to_string(),
                    Map {
                        destination: destination.to_string(),
                        left: left.to_string(),
                        right: right.to_string(),
                    },
                ))
            } else {
                None
            }
        })
        .filter_map(|map| map.ok_or(()).ok())
        .collect();

    (instruction, maps)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_input() -> Vec<String> {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        input.lines().map(|l| l.to_string()).collect()
    }

    fn test_input2() -> Vec<String> {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        input.lines().map(|l| l.to_string()).collect()
    }

    fn test_input_part2() -> Vec<String> {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        input.lines().map(|l| l.to_string()).collect()
    }

    #[test]
    fn test_parsing_input() {
        let input = test_input2();
        let (instruction, maps) = parse_input(&input);

        assert_eq!(instruction, "LLR".to_string());

        let expected = HashMap::from([
            (
                "AAA".to_string(),
                Map {
                    destination: "AAA".to_string(),
                    left: "BBB".to_string(),
                    right: "BBB".to_string(),
                },
            ),
            (
                "BBB".to_string(),
                Map {
                    destination: "BBB".to_string(),
                    left: "AAA".to_string(),
                    right: "ZZZ".to_string(),
                },
            ),
            (
                "ZZZ".to_string(),
                Map {
                    destination: "ZZZ".to_string(),
                    left: "ZZZ".to_string(),
                    right: "ZZZ".to_string(),
                },
            ),
        ]);

        assert_eq!(maps, expected);
    }

    #[test]
    fn test_part1_with_test_input() {
        let input = test_input();
        let (instruction, maps) = parse_input(&input);
        let result = part1(&instruction, &maps);
        assert_eq!(result, 2);

        let input = test_input2();
        let (instruction, maps) = parse_input(&input);
        let result = part1(&instruction, &maps);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_part1() {
        let input = day8_input();
        let (instruction, maps) = parse_input(&input);
        let result = part1(&instruction, &maps);
        assert_eq!(result, 15989);
    }

    #[test]
    fn test_part2_with_test_input() {
        let input = test_input_part2();
        let (instruction, maps) = parse_input(&input);
        let result = part2(&instruction, &maps);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_part2() {
        let input = day8_input();
        let (instruction, maps) = parse_input(&input);
        let result = part2(&instruction, &maps);
        assert_eq!(result, 13830919117339);
    }
}
