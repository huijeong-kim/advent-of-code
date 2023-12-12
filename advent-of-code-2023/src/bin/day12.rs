use std::fs::File;
use std::io::{BufRead, BufReader};

fn day12_input() -> Vec<String> {
    let file = File::open("inputs/day12.txt").unwrap();
    BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect()
}

fn main() {
    let input = day12_input();
    let result = part1(&input);
    println!("result: {}", result);
}

#[derive(PartialEq, Debug)]
struct Value {
    c: char,
    num: u64,
}
impl Value {
    fn new(c: char, num: u64) -> Self {
        Self { c, num }
    }
}

fn brute_force(records: &str, info: &Vec<u64>) -> u64 {
    let mut combinations = vec![records.to_string()];
    let mut final_combinations = Vec::new();

    while !combinations.is_empty() {
        let record = combinations.pop().unwrap();
        if record.contains('?') {
            let comb1 = record.clone().replacen('?', "#", 1);
            let comb2 = record.clone().replacen('?', ".", 1);

            combinations.push(comb1);
            combinations.push(comb2);
        } else {
            final_combinations.push(record);
        }
    }

    final_combinations
        .iter()
        .filter(|record| matched(record, info))
        .count() as u64
}

fn matched(record: &str, info: &Vec<u64>) -> bool {
    let mut values: Vec<Value> = Vec::new();
    let mut ongoing = false;

    record.chars().for_each(|c| {
        if c == '.' {
            ongoing = false;
        } else {
            if ongoing {
                assert_eq!(values.last().unwrap().c, c);
                values.last_mut().unwrap().num += 1;
            } else {
                values.push(Value { c, num: 1 });
                ongoing = true;
            }
        }
    });

    values.iter().map(|val| val.num).collect::<Vec<_>>() == *info
}

fn part1(input: &Vec<String>) -> u64 {
    let counts = input
        .iter()
        .map(|line| {
            let split = line.split(" ").collect::<Vec<_>>();
            assert_eq!(split.len(), 2);

            let records = split[0];
            let info = split[1]
                .split(",")
                .map(|n| n.parse::<u64>().unwrap())
                .collect::<Vec<_>>();

            brute_force(records, &info)
        })
        .collect::<Vec<_>>();

    counts.iter().sum()
}

fn part2(input: &Vec<String>) -> u64 {
    // to slow.. try using dynamic programming
    let counts = input
        .iter()
        .map(|line| {
            let split = line.split(" ").collect::<Vec<_>>();
            assert_eq!(split.len(), 2);

            let records = split[0].repeat(5);
            let info = split[1]
                .repeat(5)
                .split(",")
                .map(|n| n.parse::<u64>().unwrap())
                .collect::<Vec<_>>();

            brute_force(&records, &info)
        })
        .collect::<Vec<_>>();

    counts.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_input() -> Vec<String> {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        input.lines().map(|line| line.to_string()).collect()
    }

    #[test]
    fn test_part1_with_test_input() {
        let input = test_input();
        let result = part1(&input);
        assert_eq!(result, 21);
    }

    // takes so long time
    #[test]
    #[ignore]
    fn test_part1() {
        let input = day12_input();
        let result = part1(&input);
        assert_eq!(result, 7361);
    }

    #[test]
    fn test_brute_force() {
        let test_case = vec![
            ("???.###", vec![1, 1, 3], 1),
            (".??..??...?##.", vec![1, 1, 3], 4),
            ("????.#...#...", vec![4, 1, 1], 1),
            ("????.######..#####.", vec![1, 6, 5], 4),
            ("?#?#?#?#?#?#?#?", vec![1, 3, 1, 6], 1),
            ("?###????????", vec![3, 2, 1], 10),
        ];

        for (records, info, expected) in test_case {
            let actual = brute_force(records, &info);
            assert_eq!(expected, actual, "{}", records);
        }
    }

    #[test]
    fn test_part2_with_test_input() {
        let input = test_input();
        let result = part2(&input);
        assert_eq!(result, 525152);
    }
}
