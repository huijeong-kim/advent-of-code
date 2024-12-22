use std::ptr::eq;
use advent_of_code_2024::read_from_file;

struct Equation {
    target: i64,
    operands: Vec<i64>,
}
impl Equation {
    pub fn new(line: &str) -> Self {
        let (target, operands) = line.split_once(':').unwrap();
        let operands = operands.trim().split(' ').map(|s| s.parse().unwrap()).collect();

        Self {
            target: target.parse().unwrap(),
            operands,
        }
    }
    pub fn evaluate(&self, eval_pipe: bool) -> bool {
        let result= self.operands[0];
        return self.eval(result, 1, eval_pipe)
    }

    fn eval(&self, result: i64, index: usize, eval_pipe: bool) -> bool {
        if index == self.operands.len() {
            return result == self.target;
        }

        let ret = self.eval(result + self.operands[index], index+1, eval_pipe);
        if ret {
            return ret;
        }

        let ret = self.eval(result * self.operands[index], index+1, eval_pipe);
        if ret {
            return ret;
        }

        if eval_pipe {
            let piped = Equation::pipe(result, self.operands[index]);
            let ret = self.eval(piped, index+1, eval_pipe);
            if ret {
                return ret;
            }
        }

        return false;
    }
    fn pipe(val1: i64, val2: i64) -> i64 {
        let mut str_val = val1.to_string();
        str_val += val2.to_string().as_str();

        str_val.parse().unwrap()
    }

}

fn part1(equations: &Vec<Equation>) -> i64 {
    equations.iter().map(|eq| {
        return if eq.evaluate(false) {
            eq.target
        } else {
            0
        };
    }).sum()
}

fn part2(equations: &Vec<Equation>) -> i64 {
    equations.iter().map(|eq| {
        return if eq.evaluate(true) {
            eq.target
        } else {
            0
        };
    }).sum()
}

fn parse_input(input: &str) -> Vec<Equation> {
    input.lines().map(|l| Equation::new(l)).collect()
}
fn main() -> Result<(), std::io::Error> {
    let input = read_from_file("inputs/day07.txt")?;
    let equations = parse_input(&input);
    println!("part1={}", part1(&equations));
    println!("part2={}", part2(&equations));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn test_parsing() {
        let equations = parse_input(INPUT);
        assert_eq!(equations[0].target, 190);
        assert_eq!(equations[0].operands[0], 10);
        assert_eq!(equations[0].operands[1], 19);
    }

    #[test]
    fn test_part1() {
        let equations = parse_input(INPUT);
        assert_eq!(part1(&equations), 3749);
    }

    #[test]
    fn test_part2() {
        let equations = parse_input(INPUT);
        assert_eq!(part2(&equations), 11387);
    }
}