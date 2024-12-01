use advent_of_code_2024::read_from_file;
use std::collections::HashMap;

fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let nums: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect()
        })
        .collect();

    let left: Vec<i32> = nums.iter().map(|vec| vec[0]).collect();
    let right: Vec<i32> = nums.iter().map(|vec| vec[1]).collect();

    assert_eq!(left.len(), right.len());

    (left, right)
}

fn solve1(input: &str) -> i32 {
    let (mut left, mut right) = parse_input(&input);
    left.sort();
    right.sort();

    left.iter()
        .enumerate()
        .fold(0, |acc, (idx, val)| acc + (val - right[idx]).abs())
}

fn solve2(input: &str) -> i32 {
    let (left, right) = parse_input(input);
    let mut counters = HashMap::<i32, i32>::new();
    left.iter().for_each(|&l| {
        let count: Vec<i32> = right
            .iter()
            .filter(|r| **r == l)
            .copied()
            .collect::<Vec<i32>>();
        counters.insert(l, count.len() as i32);
    });

    left.iter().fold(0, |acc, x| acc + x * counters[x])
}

fn main() -> Result<(), std::io::Error> {
    let input_filename = "inputs/day01.txt".to_string();
    let input = read_from_file(&input_filename)?;

    println!("part1 = {}", solve1(&input));
    println!("part2 = {}", solve2(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let test_input = "3   4
4   3
2   5
1   3
3   9
3   3";

        assert_eq!(solve1(test_input), 11);
    }
    #[test]
    fn part2() {
        let test_input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!(solve2(test_input), 31);
    }
}
