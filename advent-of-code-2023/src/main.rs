use advent_of_code_2023::{day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12, read_from_file};
use std::env;

const MAX_DAY: i32 = 12;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("run with day number. e.g.,) day01")
    }

    if args[1] == "all" {
        for day_num in 1..MAX_DAY + 1 {
            run(day_num);
        }
    } else {
        let day_num = args[1]
            .chars()
            .filter(|c| c.is_numeric())
            .collect::<String>()
            .parse::<i32>()
            .unwrap();

        run(day_num);
    }
}

fn run(day_num: i32) {
    let input_filename = format!("inputs/day{:02}.txt", day_num);
    let input = read_from_file(&input_filename);

    println!("Run day{:02} solution =====================", day_num);
    match day_num {
        1 => day01::solution(input),
        2 => day02::solution(input),
        3 => day03::solution(input),
        4 => day04::solution(input),
        5 => day05::solution(input),
        6 => day06::solution(input),
        7 => day07::solution(input),
        8 => day08::solution(input),
        9 => day09::solution(input),
        10 => day10::solution(input),
        11 => day11::solution(input),
        12 => day12::solution(input),
        _ => panic!("not yet solved"),
    };
}
