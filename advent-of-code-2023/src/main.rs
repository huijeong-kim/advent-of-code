use std::env;
use advent_of_code_2023::read_from_file;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("run with day number. e.g.,) day01")
    }

    if args[1] == "all" {
        for day_num in 1..1 {
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

    match day_num {
        _ => panic!("not yet solved"),
    };
}
