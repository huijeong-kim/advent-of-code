use std::collections::BTreeSet;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;

fn main() {
    let rucksacks = get_day3_input();

    let priority_map = create_priority_map();
    let priorities: i32 = rucksacks
        .iter()
        .map(|v| {
            let (first, second) = get_compartment(v);
            let (first_chars, second_chars) = (string_to_btreeset(&first), string_to_btreeset(&second));

            let common: Vec<char> = first_chars.intersection(&second_chars).cloned().collect();
            priority_map.get(&common[0]).unwrap()
        })
        .sum();
    println!("{}", priorities);

    let group_proirities: i32 = rucksacks
        .chunks(3)
        .map(|chunk| {
            let first_chars = string_to_btreeset(&chunk[0]);
            let second_chars = string_to_btreeset(&chunk[1]);
            let third_chars = string_to_btreeset(&chunk[2]);

            let temp = &first_chars.intersection(&second_chars).cloned().collect();
            let common: Vec<char> = vec_to_btreeset(temp)
                .intersection(&third_chars)
                .cloned()
                .collect();

            priority_map.get(&common[0]).unwrap()
        })
        .sum();
    println!("{}", group_proirities);
}

fn string_to_btreeset(charvec: &String) -> BTreeSet<char> {
    charvec.chars().collect()
}

fn vec_to_btreeset(charvec: &Vec<char>) -> BTreeSet<char> {
    charvec.iter().map(|t| t.clone()).collect()
}

fn get_compartment(rucksack: &String) -> (String, String) {
    let split = rucksack
        .chars()
        .collect::<Vec<char>>()
        .chunks(rucksack.len() / 2)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<String>>();
    (split[0].clone(), split[1].clone())
}

fn create_priority_map() -> HashMap<char, i32> {
    let alphabet = String::from_utf8((b'a'..=b'z').chain(b'A'..=b'Z').collect()).unwrap();

    let mut result = HashMap::new();
    for (i, ch) in alphabet.chars().enumerate() {
        result.insert(ch, i as i32 + 1);
    }

    result
}

fn get_day3_input() -> Vec<String> {
    let file = File::open("inputs/day3.txt").unwrap();
    let lines = std::io::BufReader::new(file).lines();
    lines.into_iter().map(|line| line.unwrap()).collect()
}
