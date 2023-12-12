use std::collections::{BTreeMap, HashMap};

pub fn solution(input: String) {
    // Part 1
    let numbers: Vec<u32> = input
        .lines()
        .map(|line| {
            let numbers: String = line.chars().filter(|&c| !c.is_alphabetic()).collect();
            let number = numbers[0..1].to_string() + &numbers[numbers.len() - 1..];
            let number: u32 = number.parse().unwrap();
            number
        })
        .collect();

    println!("part1: {}", numbers.iter().sum::<u32>());

    // Part 2
    let number_map: HashMap<&str, u32> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let results: Vec<_> = input
        .lines()
        .map(|line| {
            // find numbers in alphabet
            let result1: Vec<_> = number_map
                .iter()
                .flat_map(|(c, n)| {
                    let matches: Vec<_> = line.match_indices(c).collect();
                    matches
                        .into_iter()
                        .map(|(idx, _)| (idx as u32, *n))
                        .collect::<Vec<(u32, u32)>>()
                })
                .collect();

            // find numbers
            let result2: Vec<_> = line
                .chars()
                .enumerate()
                .filter(|&(_, c)| c.is_numeric())
                .map(|(idx, c)| (idx as u32, c as u32 - '0' as u32))
                .collect();

            let mut numbers: BTreeMap<u32, u32> = BTreeMap::new();
            let result: Vec<_> = result1.into_iter().chain(result2).collect();
            result.iter().for_each(|(a, b)| {
                numbers.insert(*a, *b);
            });

            let first = numbers.iter().nth(0).unwrap().1;
            let last = &numbers.iter().last().unwrap().1;

            first.to_string() + &last.to_string()
        })
        .collect();

    println!(
        "part2: {}",
        results
            .iter()
            .map(|n| n.parse::<u32>().unwrap())
            .sum::<u32>()
    );
}
