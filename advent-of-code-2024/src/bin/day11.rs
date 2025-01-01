use advent_of_code_2024::read_from_file;

fn main() {
    let input = read_from_file("inputs/day11.txt").unwrap();
    println!("part1={}", part1(&input));
    println!("part2={}", part2(&input)); // timeout
}

fn trim_leading_zeros(num: &str) -> String {
    let num = num.trim_start_matches('0');
    return if num == "" {
        "0".to_string()
    } else {
        num.to_string()
    };
}
fn split_num(num: &String) -> (String, String) {
    let (left, right) = num.split_at(num.len() / 2);
    (trim_leading_zeros(left), trim_leading_zeros(right))
}

fn multiply(num: &String, val: i64) -> String {
    let int: i64 = num.parse().unwrap();
    (int * val).to_string()
}
fn blink(nums: &Vec<String>) -> Vec<String> {
    nums.into_iter()
        .flat_map(|num| {
            if num == "0" {
                vec!["1".to_string()]
            } else if num.len() % 2 == 0 {
                let (left, right) = split_num(num);
                vec![left, right]
            } else {
                vec![multiply(num, 2024)]
            }
        })
        .collect()
}

fn blinks(input: &str, num_times: i32) -> usize {
    let mut stones: Vec<String> = input.split_whitespace().map(|s| s.to_string()).collect();
    for i in 0..num_times {
        stones = blink(&stones);

        if i % 5 == 0 {
            println!("completed 5");
            println!("{:?}", stones);
        }
    }
    stones.len()
}

fn part1(input: &str) -> usize {
    blinks(input, 25)
}

fn part2(input: &str) -> usize {
    blinks(input, 75)
}
#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "125 17";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 55312);
    }
}
