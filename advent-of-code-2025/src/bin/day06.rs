use advent_of_code_2025::read_from_file;

fn main() -> Result<(), std::io::Error> {
    let input = read_from_file("inputs/day06.txt")?;
    println!("part1={}", part1(&input));
    println!("part2={}", part2(&input));
    Ok(())
}

fn part1(input: &str) -> i64 {
    let problem = Problem::from(&input);
    problem.solve()
}

fn part2(input: &str) -> i64 {
    let problem = Problem::from_right_to_left(&input);
    problem.solve()
}

#[derive(Default, Debug)]
struct Problem {
    numbers: Vec<Vec<i64>>,
    operators: Vec<char>,
    cols: usize,
    rows: usize,
}

impl Problem {
    fn parse_operator(input: &str) -> Problem {
        let mut problem = Problem::default();
        let operators: Vec<&str> = input.lines().last().unwrap().split_whitespace().collect();
        problem.cols = operators.len();
        problem.rows = input.lines().count() - 1;
        
        problem.numbers.resize(problem.cols, Vec::new());
        operators.iter().for_each(|op| {
            problem.operators.push(op.chars().nth(0).unwrap());
        });

        problem
    }

    fn from(input: &str) -> Problem {
        let mut problem = Problem::parse_operator(input);
        
        for line_idx in 0..problem.rows {
            let line = input.lines().nth(line_idx).unwrap();
            let cols: Vec<&str> = line.split_whitespace().collect();
            cols.iter().enumerate().for_each(|(idx, num)| {
                problem.numbers[idx].push(num.parse().unwrap());    
            });
        }

        problem
    }

    fn from_right_to_left(input: &str) -> Self {
        let mut problem = Problem::parse_operator(input);
        
        let mut char_idx = 0;
        let mut col_idx = 0;
        
        let line_len = input.lines().nth(0).unwrap().len();

        while char_idx < line_len {
            let mut number = String::new();
            for line_no in 0..problem.rows {
                number.push(input.lines().nth(line_no).unwrap().chars().nth(char_idx).unwrap());
            }

            if number.chars().all(|ch| ch == ' ') {
                col_idx += 1;
                char_idx += 1;
                continue;
            }

            let number = trim_number(&number);
            problem.numbers[col_idx].push(number);
            
            char_idx += 1;
        }
        
        problem
    }
    fn solve(&self) -> i64 {
        self.operators.iter().enumerate().map(|(idx, op)| {
            let initial_value = if *op == '+' { 0 } else if *op == '*' { 1 } else { panic!("can't reach here")};
            self.numbers[idx].iter().fold(initial_value, |acc, x| {
                match op {
                    '+' => acc + x,
                    '*' => acc * x,
                    _ => panic!("not expected")
                }
            })
        }).sum()
    }
}

fn trim_number(number: &String) -> i64 {
    // 100 => 1
    // 001 => 1

    let mut start_idx = 0;
    let mut end_idx = number.len() - 1;

    while start_idx < number.len() && number.chars().nth(start_idx).unwrap() == ' ' {
        start_idx += 1;
    }

    while end_idx > 0 && number.chars().nth(end_idx).unwrap() == ' ' {
        if end_idx != 0 {
            end_idx -= 1;
        }        
    }

    number[start_idx..end_idx + 1].parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = 
"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   + ";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 4277556);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 3263827);
    }
}