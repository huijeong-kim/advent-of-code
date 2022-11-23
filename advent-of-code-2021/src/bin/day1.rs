use std::fs::File;
use std::io::BufRead;
use std::path::PathBuf;

fn main() {
    let numbers = get_day1_input();

    println!("PART1");
    println!("Result: {}, {}", part1(&numbers), part1_windows(&numbers));

    println!("PART2");
    println!("Result: {}, {}", part2(&numbers), part2_windows(&numbers));
}

fn get_day1_input() -> Vec<i32> {
    let input = PathBuf::from("inputs/day1.txt");
    let file = File::open(input).unwrap();
    let lines = std::io::BufReader::new(file).lines();

    lines
        .into_iter()
        .map(|n| n.unwrap().parse::<i32>().unwrap())
        .collect()
}

fn part1(numbers: &Vec<i32>) -> i32 {
    let mut prev = i32::MAX;

    numbers.iter().fold(0i32, |sum, val| {
        let result = if val > &prev { sum + 1 } else { sum };
        prev = *val;
        return result;
    })
}

fn part1_windows(numbers: &Vec<i32>) -> i32 {
    numbers.windows(2).filter(|x| x[0] < x[1]).count() as i32
}

fn part2(numbers: &Vec<i32>) -> i32 {
    let mut prev = i32::MAX;

    let mut sums = Vec::new();
    for i in 0..numbers.len() - 2 {
        sums.push(numbers[i] + numbers[i + 1] + numbers[i + 2]);
    }

    sums.iter().fold(0i32, |sum, val| {
        let result = if val > &prev { sum + 1 } else { sum };
        prev = *val;
        return result;
    })
}

fn part2_windows(numbers: &Vec<i32>) -> i32 {
    let sums: Vec<i32> = numbers.windows(3).map(|x| x[0] + x[1] + x[2]).collect();
    sums.windows(2).filter(|x| x[0] < x[1]).count() as i32
}
