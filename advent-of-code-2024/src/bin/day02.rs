use advent_of_code_2024::*;

fn get_reports(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|str| {
            str.split_whitespace()
                .map(|val| val.parse().unwrap())
                .collect()
        })
        .collect()
}


fn is_in_range(val1: i32, val2: i32) -> bool {
    let diff = (val1 - val2).abs();
    diff >= 1 && diff <= 3
}

fn is_safe_part1(report: &Vec<i32>) -> bool {
    let mut sorted = report.clone();
    sorted.sort();
    let is_ascending = is_same_vec(&sorted, report);

    sorted.sort_by(|a, b| b.cmp(a));
    let is_descending = is_same_vec(&sorted, report);

    if is_ascending ^ is_descending {
        for i in 1..sorted.len() {
            if !is_in_range(sorted[i], sorted[i - 1]) {
                return false;
            }
        }
        return true;
    }

    false
}

#[warn(dead_code)]
fn is_safe_part1_v1(report: &Vec<i32>) -> bool {
    assert!(report.len() >= 2);

    let mut prev = report[0];
    let is_increasing = report[0] < report[1];

    for i in 1..report.len() {
        if !is_in_range(report[i], prev) {
            return false;
        }

        if is_increasing && prev > report[i] {
            return false;
        }
        if !is_increasing && prev < report[i] {
            return false;
        }

        prev = report[i];
    }

    true
}

fn is_safe_part2(report: &Vec<i32>) -> bool {
    if is_safe_part1(report) {
        return true;
    }

    for i in 0..report.len() {
        let partial: Vec<i32> = report
            .iter()
            .enumerate()
            .filter(|(idx, _val)| *idx != i)
            .map(|(_idx, val)| *val)
            .collect();

        if is_safe_part1(&partial) {
            return true;
        }
    }

    false
}

fn scan_report(input: &str, is_safe: fn(&Vec<i32>) -> bool) -> usize {
    let reports = get_reports(input);
    reports
        .iter()
        .filter(|r| {
            return is_safe(*r);
        })
        .count()
}

fn part1(input: &str) -> usize {
    scan_report(&input, is_safe_part1)
}
fn part2(input: &str) -> usize {
    scan_report(&input, is_safe_part2)
}
fn main() -> Result<(), std::io::Error> {
    let input = advent_of_code_2024::read_from_file("inputs/day02.txt")?;
    println!("part1 = {}", part1(&input));
    println!("part2 = {}", part2(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 4);
    }
}
