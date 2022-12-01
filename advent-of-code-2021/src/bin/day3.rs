use std::fs::File;
use std::io::BufRead;
use std::path::PathBuf;

fn main() {
    let test_inputs = vec![
        "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001",
        "00010", "01010",
    ];
    let test_inputs = test_inputs.iter().map(|s| String::from(*s)).collect();
    let (gamma, epsilon) = part1(&test_inputs);
    println!(
        "gamma: {}, epsilon: {} => result {}",
        gamma,
        epsilon,
        gamma * epsilon
    );

    let signals = get_day3_input();
    let (gamma, epsilon) = part1(&signals);
    println!(
        "gamma: {}, epsilon: {} => result {}",
        gamma,
        epsilon,
        gamma * epsilon
    );
}

fn get_day3_input() -> Vec<String> {
    let input = PathBuf::from("inputs/day3.txt");
    let file = File::open(input).unwrap();
    let lines = std::io::BufReader::new(file).lines();

    lines
        .into_iter()
        .map(|n| n.unwrap().parse::<String>().unwrap())
        .collect()
}

fn part1(inputs: &Vec<String>) -> (i32, i32) {
    let num_digits = inputs[0].len();
    let mut counts = vec![0; num_digits];

    for num in inputs {
        let bits: Vec<u32> = num.chars().map(|c| c.to_digit(2).unwrap()).collect();

        for (i, val) in bits.into_iter().enumerate() {
            counts[i] += if val == 1 {
                1
            } else {
                -1
            };
        }
    }

    let mut gamma = String::new();
    let mut epsilon = String::new();
    for n in counts {
        if n > 0 {
            gamma.push('1');
            epsilon.push('0');
        } else {
            gamma.push('0');
            epsilon.push('1')
        }
    };

    let gamma = i32::from_str_radix(&gamma, 2).unwrap();
    let epsilon = i32::from_str_radix(&epsilon, 2).unwrap();

    (gamma, epsilon)
}

fn part2(inputs: &Vec<String>) -> (i32, i32) {
    let num_digits = inputs[0].len();
    let mut counts = vec![0; num_digits];

    for num in inputs {
        let bits: Vec<u32> = num.chars().map(|c| c.to_digit(2).unwrap()).collect();

        for (i, val) in bits.into_iter().enumerate() {
            counts[i] += if val == 1 {
                1
            } else {
                -1
            };
        }
    }

    let mut inputs = inputs.clone();
    let mut index = 0;
    while inputs.len() == 1 {
        inputs = inputs.iter().filter(|s| {
            (*s)[index] == '1'
        }).collect();
    }

    println!("inputs: {:?}", inputs);
    let oxygen_generator = 0;
    let co2_scrubber = 0;

    (oxygen_generator, co2_scrubber)
}