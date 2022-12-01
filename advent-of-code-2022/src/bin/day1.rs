use std::fs;

fn main() {
    let calories = get_day1_input();
    let calories: Vec<i32> = calories.iter().map(|vec| vec.iter().sum()).collect();

    let top_calories = calories.iter().max().unwrap();
    println!("{}", top_calories);

    let sorted = get_sorted(&calories);
    let top_3_elves = sorted.chunks(3).last().unwrap();
    let top_3_sum = top_3_elves.iter().sum::<i32>();
    println!("{}", top_3_sum);
}

fn get_day1_input() -> Vec<Vec<i32>> {
    let content = fs::read_to_string("inputs/day1.txt").expect("Failed to open input file");

    content.split("\n").fold(Vec::new(), |mut acc, data| {
        if data == "" {
            acc.push(Vec::new());
        } else {
            if acc.is_empty() || data == "" {
                acc.push(Vec::new());
            }
            acc.last_mut()
                .unwrap()
                .push(data.parse::<i32>().expect("Failed to parse str"));
        }
        acc
    })
}

fn get_sorted(vec: &Vec<i32>) -> Vec<i32> {
    let mut new_vec = vec.clone();
    new_vec.sort();

    new_vec
}
