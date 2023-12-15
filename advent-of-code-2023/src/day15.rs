use std::fmt::{Debug, Formatter};

pub fn solution(input: String) {
    let result = part1(&input);
    println!("part1: {result}");
    let result = part2(&input);
    println!("part2: {result}");
}

fn part1(input: &str) -> i32 {
    let steps = parse_to_steps(input);
    steps.iter().map(|value| run_steps(value)).sum()
}

fn parse_to_steps(input: &str) -> Vec<String> {
    input.split(",").map(|s| s.to_string()).collect::<Vec<_>>()
}

fn run_steps(input: &str) -> i32 {
    input
        .chars()
        .fold(0, |acc, c| ((acc + c as i32) * 17) % 256)
}

#[derive(Default, Clone)]
struct Box {
    lenses: Vec<(String, i32)>,
}
impl Debug for Box {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for l in &self.lenses {
            write!(f, "[{} {}] ", l.0, l.1).unwrap();
        }
        write!(f, "")
    }
}
impl Box {
    fn new() -> Self {
        Self { lenses: Vec::new() }
    }
    fn contains(&self, label: &str) -> bool {
        !self.lenses.iter().all(|l| l.0 != label)
    }
    fn remove(&mut self, label: &str) {
        let target_idx = self
            .lenses
            .iter()
            .enumerate()
            .find(|(idx, l)| l.0 == label)
            .unwrap()
            .0;

        self.lenses.remove(target_idx);
    }

    fn add(&mut self, label: &str, focal_length: i32) {
        self.lenses.push((label.to_string(), focal_length));
    }
    fn replace(&mut self, label: &str, focal_length: i32) {
        self.lenses.iter_mut().enumerate().for_each(|(idx, l)| {
            if l.0 == label {
                l.1 = focal_length;
            }
        });
    }
}

fn parse_equal_sign(value: &str) -> (&str, i32, usize) {
    let label = value.split("=").nth(0).unwrap();
    let focal_length = value.split("=").nth(1).unwrap().parse::<i32>().unwrap();
    let box_num = run_steps(label) as usize;

    (label, focal_length, box_num)
}
fn parse_dash_sign(value: &str) -> (&str, usize) {
    let label = value.split("-").nth(0).unwrap();
    let box_num = run_steps(label) as usize;

    (label, box_num)
}

fn part2(input: &str) -> i32 {
    let mut boxes = vec![Box::new(); 256];

    let steps = parse_to_steps(&input);
    steps.iter().for_each(|value| {
        if value.contains('=') {
            let (label, focal_length, box_num) = parse_equal_sign(value);

            if boxes[box_num].contains(label) {
                boxes[box_num].replace(label, focal_length);
            } else {
                boxes[box_num].add(label, focal_length);
            }
        } else if value.contains('-') {
            let (label, box_num) = parse_dash_sign(value);

            if boxes[box_num].contains(label) {
                boxes[box_num].remove(label);
            }
        } else {
            unreachable!();
        }
    });

    boxes
        .iter()
        .enumerate()
        .map(|(box_num, b)| {
            b.lenses
                .iter()
                .enumerate()
                .map(move |(l_num, l)| (box_num as i32 + 1) * (l_num as i32 + 1) * l.1)
        })
        .flatten()
        .sum::<i32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_run_steps() {
        let test_cases = vec![("HASH", 52), ("rn", 0), ("qp", 1), ("cm", 0)];

        for tc in test_cases {
            assert_eq!(run_steps(tc.0), tc.1);
        }
    }

    #[test]
    fn test_part1_with_test_input() {
        let result = part1(TEST_INPUT);
        assert_eq!(result, 1320);
    }

    #[test]
    fn test_part1() {
        let input = crate::read_from_file("inputs/day15.txt");
        let result = part1(&input);
        assert_eq!(result, 521341);
    }

    #[test]
    fn test_part2_with_test_input() {
        let result = part2(TEST_INPUT);
        assert_eq!(result, 145);
    }

    #[test]
    fn test_part2() {
        let input = crate::read_from_file("inputs/day15.txt");
        let result = part2(&input);
        assert_eq!(result, 252782);
    }
}
