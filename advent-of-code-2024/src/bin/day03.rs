use advent_of_code_2024::read_from_file;

#[derive(Debug)]
enum Operation {
    Mul(MultiplyOp),
    Do,
    Dont,
}

#[derive(Debug)]
struct MultiplyOp {
    val1: i32,
    val2: i32,
}

impl MultiplyOp {
    fn execute(&self) -> i32 {
        return self.val1 * self.val2;
    }
}

struct Scanner {
    input: String,
    idx: usize,
}

impl Scanner {
    fn new(input: String) -> Scanner {
        Scanner { input, idx: 0 }
    }

    fn find_next_op(&mut self) -> Option<Operation> {
        while self.idx < self.input.len() {
            let partial = &self.input[self.idx..];
            if partial.starts_with("mul") {
                self.idx += "mul".len();
                if let Some(op) = self.is_valid_multiply() {
                    return Some(Operation::Mul(op));
                }
            } else if partial.starts_with("do()") {
                self.idx += "do()".len();
                return Some(Operation::Do);
            } else if partial.starts_with("don't()") {
                self.idx += "don't()".len();
                return Some(Operation::Dont);
            } else {
                self.idx += 1;
            }
        }
        None
    }

    fn advance_and_get_numstr(&mut self) -> String {
        let mut num = String::new();
        while self.idx < self.input.len() {
            let ch = self.input.chars().nth(self.idx).unwrap();
            if ch.is_numeric() {
                num.push(ch);
                self.idx += 1;
            } else {
                break;
            }
        }
        return num;
    }

    fn ended(&self) -> bool {
        self.idx >= self.input.len()
    }

    fn advance_and_equal(&mut self, ch: char) -> bool {
        if self.ended() {
            return false;
        }

        let current = self.input.chars().nth(self.idx).unwrap();
        self.idx += 1;

        ch == current
    }

    fn is_valid_multiply(&mut self) -> Option<MultiplyOp> {
        if !self.advance_and_equal('(') {
            return None;
        }

        let num1 = self.advance_and_get_numstr();
        if num1.parse::<i32>().is_err() {
            return None;
        }

        if !self.advance_and_equal(',') {
            return None;
        }

        let num2 = self.advance_and_get_numstr();
        if num2.parse::<i32>().is_err() {
            return None;
        }

        if !self.advance_and_equal(')') {
            return None;
        }

        Some(MultiplyOp {
            val1: num1.parse().unwrap(),
            val2: num2.parse().unwrap(),
        })
    }
}

fn part1(input: String) -> i32 {
    let mut result = 0;
    let mut scan = Scanner::new(input);
    while let Some(op) = scan.find_next_op() {
        match op {
            Operation::Mul(multiply_op) => {
                result += multiply_op.execute();
            }
            _ => continue,
        }
    }

    result
}

fn part2(input: String) -> i32 {
    let mut result = 0;
    let mut can_execute = true;
    let mut scan = Scanner::new(input);
    while let Some(op) = scan.find_next_op() {
        match op {
            Operation::Mul(multiply_op) => {
                if can_execute {
                    result += multiply_op.execute();
                }
            }
            Operation::Do => {
                can_execute = true;
            }
            Operation::Dont => {
                can_execute = false;
            }
        }
    }

    result
}

fn main() -> Result<(), std::io::Error> {
    let input = read_from_file("inputs/day03.txt")?;
    println!("part1 = {}", part1(input.clone()));
    println!("part2 = {}", part2(input.clone()));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const INPUT2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    #[test]
    fn test_part1() {
        assert_eq!(part1(String::from(INPUT)), 161);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(String::from(INPUT2)), 48);
    }
}
