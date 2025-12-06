use advent_of_code_2025::read_from_file;

fn main() -> Result<(), std::io::Error> {
    let input = read_from_file("inputs/day02.txt")?;
    let input = parse_input(&input);
    println!("part1={}", part1(&input));
    println!("part2={}", part2(&input));
    Ok(())
}

#[derive(Debug)]
struct Range {
    start: String,
    end: String,
}

fn smallest_num(n: u32) -> i64 {
    // 10 ^ n
    let sub = if n == 1 {1} else {0};
    10_i64.pow(n - 1) - sub

}

fn largest_num(n: u32) -> i64 {
    // 10 ^ (n+1) - 1
    10_i64.pow(n) - 1
}

fn is_duplicate(val: i64) -> bool {
    let str_val = val.to_string();
    let idx = str_val.len() / 2;

    for i in 0..idx {
        if str_val.chars().nth(i) != str_val.chars().nth(i + idx) {
            return false;
        }
    }

    true
}

fn is_repeating(val: i64) -> bool {
    let str_val = val.to_string();

    for i in 1..str_val.len() {
        let substr = &str_val[0..i];
        if str_val == substr.repeat(str_val.len() / i) {
            return true;
        }
    }

    false
}

impl Range {
    fn find_duplicate_ids(&self) -> Vec<i64> {
        let mut invalid_ids = Vec::new();
        let start_len = self.start.len();
        let end_len = self.end.len();

        for i in start_len..end_len+1 {
            if i % 2 != 0 {
                continue;
            }
            let start_num: i64 = if i == start_len { self.start.parse().unwrap() } else { smallest_num(i as u32) };
            let end_num: i64 = if i == end_len { self.end.parse().unwrap() } else { largest_num(i as u32) };

            println!("  {start_num}..{end_num}");
            for val in start_num..end_num+1 {
                if is_duplicate(val) {
                    invalid_ids.push(val);
                }
            }

        }

        invalid_ids
    }

    fn find_repeating_ids(&self) -> Vec<i64> {
        let mut invalid_ids = Vec::new();
        let start_len = self.start.len();
        let end_len = self.end.len();

        for i in start_len..end_len+1 {
            let start_num: i64 = if i == start_len { self.start.parse().unwrap() } else { smallest_num(i as u32) };
            let end_num: i64 = if i == end_len { self.end.parse().unwrap() } else { largest_num(i as u32) };

            for val in start_num..end_num+1 {
                if is_repeating(val) {
                    invalid_ids.push(val);
                }
            }

        }

        invalid_ids
    }
}

fn parse_input(input: &str) -> Vec<Range> {
    input.split(',').map(|range| {
        let val: Vec<&str> = range.split('-').collect();
        Range {
            start: val[0].to_string(),
            end: val[1].to_string(),
        }
    }).collect()
}

fn part1(range: &Vec<Range>) -> i64 {
    range.iter().map(|r| {
        r.find_duplicate_ids().iter().copied().sum::<i64>()
    }).sum::<i64>()
}

fn part2(range: &Vec<Range>) -> i64 {
    range.iter().map(|r| {
        r.find_repeating_ids().iter().copied().sum::<i64>()
    }).sum::<i64>()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_part1() {
        let input = parse_input(INPUT);
        assert_eq!(part1(&input), 1227775554);
    }

    #[test]
    fn test_part2() {
        let input = parse_input(INPUT);
        assert_eq!(part2(&input), 4174379265);
    }

    #[test]
    fn test_util() {
        assert_eq!(smallest_num(1), 0);
        assert_eq!(largest_num(1), 9);

        assert_eq!(smallest_num(2), 10);
        assert_eq!(largest_num(2), 99);

        assert_eq!(smallest_num(3), 100);
        assert_eq!(largest_num(3), 999);
    }

    #[test]
    fn test_find_duplicate_ids() {
        let range = Range {
            start: "11".to_string(),
            end: "22".to_string(),
        };
        let invalid_ids = range.find_duplicate_ids();
        assert_eq!(invalid_ids.len(), 2);
        assert!(invalid_ids.contains(&11));
        assert!(invalid_ids.contains(&22));

        let range = Range {
            start: "95".to_string(),
            end: "115".to_string(),
        };
        let invalid_ids = range.find_duplicate_ids();
        assert!(invalid_ids.contains(&99));
    }

    #[test]
    fn test_find_repeating_ids() {
        let range = Range {
            start: "95".to_string(),
            end: "115".to_string(),
        };
        let invalid_ids = range.find_repeating_ids();
        assert!(invalid_ids.contains(&99));
        assert!(invalid_ids.contains(&111));

        let range =  Range {
            start: "998".to_string(),
            end: "1012".to_string(),
        };
        let invalid_ids = range.find_repeating_ids();
        assert!(invalid_ids.contains(&999));
        assert!(invalid_ids.contains(&1010));
        
        let range =  Range {
            start: "1188511880".to_string(),
            end: "1188511890".to_string(),
        };
        let invalid_ids = range.find_repeating_ids();
        assert_eq!(invalid_ids.len(), 1);
        assert!(invalid_ids.contains(&1188511885));

        let range =  Range {
            start: "1698522".to_string(),
            end: "1698528".to_string(),
        };
        let invalid_ids = range.find_repeating_ids();
        assert_eq!(invalid_ids.len(), 0);

        let range =  Range {
            start: "824824821".to_string(),
            end: "824824827".to_string(),
        };
        let invalid_ids = range.find_repeating_ids();
        assert_eq!(invalid_ids.len(), 1);
        assert!(invalid_ids.contains(&824824824));

    }

}