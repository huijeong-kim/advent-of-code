use advent_of_code_2025::read_from_file;

fn main() -> Result<(), std::io::Error> {
    let input = read_from_file("inputs/day03.txt")?;
    println!("part1={}", part1(&input));
    println!("part2={}", part2(&input));
    Ok(())
}

fn part1(input: &str) -> i64 {
    input.lines().map(|line| {
        find_largest_joltage(line)
    }).sum()
}

fn part2(input: &str) -> i64 {
    input.lines().map(|line| {
        find_twelve_sum(line)
    }).sum()
}
fn to_num(ch: &char) -> i64 {
    ch.to_digit(10).unwrap() as i64
}

fn find_largest_joltage(bank: &str) -> i64 {
    // bruteforce
    let mut largest: i64 = 0;
    for i in 0..bank.len() {
        for j in i+1..bank.len() {
            let new_sum = to_num(&bank.chars().nth(i).unwrap()) * 10 + to_num(&bank.chars().nth(j).unwrap());
            if new_sum > largest {
                largest = new_sum;
            }
        }
    }

    largest
}

fn find_twelve_sum(bank: &str) -> i64 {
    // stack + greedy
    let mut to_remove = bank.len() - 12;
    let mut stack = Vec::new();

    for ch in bank.chars() {
        let cur_num = to_num(&ch);

        while to_remove > 0 {
            if let Some(&last_num) = stack.last() {
                if last_num < cur_num {
                    stack.pop();
                    to_remove -= 1;
                }
            } else {
                break;
            }
        }
        stack.push(cur_num);
    }

    let mut num = 0;
    for i in 0..12 {
        num = num*10 + stack[i];
    }
    num
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 357);
    }   

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 3121910778619);
    }   

    #[test]
    fn test_find_largest_joltage() {
        assert_eq!(find_largest_joltage("987654321111111"), 98);
        assert_eq!(find_largest_joltage("811111111111119"), 89);
    }
}