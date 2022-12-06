use std::collections::HashSet;
use std::fs;

fn main() {
    let input = get_day6_input();

    println!("{}", get_result(&input, 4));
    println!("{}", get_result(&input, 14));
}

fn get_result(input: &Vec<char>, num: i32) -> i32 {
    for (i, iter) in input.windows(num as usize).into_iter().enumerate() {
        let set: HashSet<&char> = iter.into_iter().collect();
        if set.len() == num as usize {
            return i as i32 + num;
        }
    }
    return 0;
}

fn get_day6_input() -> Vec<char> {
    let str = fs::read_to_string("inputs/day6.txt").expect("Failed to open input file");
    str.chars().collect()
}
