use std::collections::HashMap;
use advent_of_code_2024::read_from_file;

fn get_mid_value(vec: &Vec<i32>) -> i32 {
    vec[vec.len() / 2]
}

struct Rule {
    before: Vec<i32>,
    after: Vec<i32>,
}

#[derive(Default)]
struct Manual {
    rules: HashMap<i32, Rule>,
    pages: Vec<Vec<i32>>,
}

impl Manual {
    fn new(input: &str) -> Self {
        let (rules, pages) = input.split_once("\n\n").unwrap();

        let rules: Vec<(i32, i32)> = rules
            .lines()
            .map(|line| line.split_once('|').unwrap())
            .map(|val| (val.0.parse().unwrap(), val.1.parse().unwrap()))
            .collect();
        let pages = pages
            .lines()
            .map(|line| line.split(',').map(|v| v.parse().unwrap()).collect())
            .collect();

        let mut rules_map: HashMap<i32, Rule> = HashMap::new();
        for rule in &rules {
            if let Some(found) = rules_map.get_mut(&rule.0) {
                found.after.push(rule.1);
            } else {
                rules_map.insert(rule.0, Rule { before: Vec::new(), after: vec![rule.1] });
            }

            if let Some(found) = rules_map.get_mut(&rule.1) {
                found.before.push(rule.0);
            } else {
                rules_map.insert(rule.1, Rule { before: vec![rule.0], after: Vec::new()});
            }
        }

        Manual {rules: rules_map, pages }
    }

    fn is_valid_pair(&self, num0: i32, num1: i32) -> bool {
        if let Some(entry) = self.rules.get(&num0) {
            if entry.after.contains(&num1) { return true; }
        }

        if let Some(entry) = self.rules.get(&num1) {
            if entry.before.contains(&num0) { return true; }
        }

        return false;
    }

    fn valid(&self, pages: &Vec<i32>) -> bool {
        pages
            .windows(2)
            .all(|nums| self.is_valid_pair(nums[0], nums[1]))
    }

    fn can_be_appended(&self, num: i32, pages: &Vec<i32>) -> bool {
        for i in pages {
            if let Some(entry) = self.rules.get(&num) {
                if entry.before.contains(i) {
                    return false;
                }
            }
        }
        true
    }
    fn reorder(&self, _input: Vec<i32>) -> Vec<i32> {
        unimplemented!()
    }

    fn get_valid_list(&self) -> Vec<Vec<i32>> {
        self.pages
            .iter()
            .filter(|list| self.valid(*list))
            .map(|list| list.clone())
            .collect()
    }
    fn get_invalid_list(&self) -> Vec<Vec<i32>> {
        self.pages
            .iter()
            .filter(|list| !self.valid(*list))
            .map(|list| list.clone())
            .collect()
    }
}

fn part1(input: &str) -> i32 {
    let manual = Manual::new(input);
    let valid = manual.get_valid_list();
    valid.iter().map(|list| get_mid_value(list)).sum()
}

fn part2(input: &str) -> i32 {
    let manual = Manual::new(input);
    let invalid = manual.get_invalid_list();
    invalid
        .iter()
        .map(|list| manual.reorder(list.clone()))
        .map(|list| get_mid_value(&list))
        .sum()
}

fn main() -> Result<(), std::io::Error> {
    let input = read_from_file("inputs/day05.txt")?;
    println!("part1 = {}", part1(&input));
    println!("part2 = {}", part2(&input)); // 4822 too low
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test_parse_input() {
        let manual = Manual::new(INPUT);
        assert!(manual.rules.get(&47).unwrap().after.contains(&53));
        assert!(manual.rules.get(&53).unwrap().before.contains(&47));

        assert_eq!(
            manual.pages,
            vec![
                vec![75, 47, 61, 53, 29],
                vec![97, 61, 53, 29, 13],
                vec![75, 29, 13],
                vec![75, 97, 47, 61, 53],
                vec![61, 13, 29],
                vec![97, 13, 75, 29, 47]
            ]
        );
    }

    #[test]
    fn test_reorder() {
        let manual = Manual::new(INPUT);
        assert_eq!(manual.reorder(vec![75, 97, 47, 61, 53]), vec![97, 75, 47, 61, 53]);
        assert_eq!(manual.reorder(vec![61, 13, 29]), vec![61, 29, 13]);
        assert_eq!(
            manual.reorder(vec![97, 13, 75, 29, 47]),
            vec![97, 75, 47, 29, 13]
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 143);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 123);
    }
}
