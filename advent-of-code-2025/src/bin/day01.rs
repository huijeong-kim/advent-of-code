use advent_of_code_2025::read_from_file;

fn main() -> Result<(), std::io::Error> {
    let input_filename = "inputs/day01.txt";
    let input = read_from_file(input_filename)?;

    let instructions = parse_input(&input);
    println!("part1={}", part1(&instructions)); // 1036
    println!("part2={}", part2(&instructions));
    Ok(())
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    fn from(input: char) -> Self {
        match input {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("invalid direction")
        }
    }
}

#[derive(Debug)]
struct Instruction {
    dir: Direction,
    count: i64,
}


fn parse_input(input: &str) -> Vec<Instruction> {
    input.lines().map(|line|
        Instruction {
            dir: Direction::from(line.chars().next().unwrap()),
            count: line[1..].parse().unwrap()
        }
    ).collect()
}

struct Value {
    val: i64,
}

impl Value {
    fn new() -> Self {
        Value::new_test(50)
    }

    fn new_test(init_val: i64) -> Self {
        Self {
            val: init_val,
        }
    }

    fn go_left_bruteforce(&mut self, count: i64) -> i64 {
        let mut zero = 0;
        let mut count = count;
        while count > 0 {
            self.val -= 1;
            count = count - 1;

            if self.val == -1 {
                self.val = 99;
            }
            if self.val == 0 {
                zero += 1;
            }
        }

        zero
    }

    fn go_right_bruteforce(&mut self, count: i64) -> i64 {
        let mut zero = 0;
        let mut count = count;
        while count > 0 {
            self.val += 1;
            count = count - 1;

            if self.val == 100{
                self.val = 0;
                zero += 1;
            }
        }

        println!("TO RIGHT {count}, passed {zero}, {}", self.val);
        zero
    }

    fn go_left(&mut self, count: i64) -> i64 {
        let mut zero = 0;
        let current_val = self.val;
        let distance = count;

        if current_val == 0 {
            zero = distance / 100;
        } else if distance < current_val {
            zero = 0;
        } else {
            zero = (distance - current_val) / 100 + 1;
        }

        self.val = (current_val - distance) % 100;
        if self.val < 0 {
            self.val += 100;
        }

        // println!("TO LEFT {count}, passed {zero}, {}", self.val);
        zero
    }

    fn go_right(&mut self, count: i64) -> i64 {
        self.val = self.val + count;
        let passed_zero = self.val / 100;
        self.val = self.val % 100;

        // println!("TO RIGHT {count}, passed {passed_zero}, {}", self.val);
        passed_zero
    }

    fn execute(&mut self, inst: &Instruction) -> i64 {
        match inst.dir {
            Direction::Left => {
                self.go_left(inst.count)
            }
            Direction::Right => {
                self.go_right(inst.count)
            }
        }
    }
}

fn part1(input: &Vec<Instruction>) -> i64 {
    let mut current = Value::new();
    input.iter().map(|inst| {
        let _ = current.execute(inst);
        current.val
    }).filter(|val| *val == 0).count() as i64
}

fn part2(input: &Vec<Instruction>) -> i64 {
    let mut current = Value::new();
    input.iter().map(|inst| {
        current.execute(inst)
    }).sum::<i64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn test_part1() {
        let instructions = parse_input(INPUT);
        assert_eq!(part1(&instructions), 3);
    }

    #[test]
    fn test_part2() {
        let instructions = parse_input(INPUT);
        assert_eq!(part2(&instructions), 6);
    }

    #[test]
    fn test_case() {
        let input = "L50
R101";

        // 50 -> 50 = 0 (+1)
        // 0 -> 101 = 101 = 1 (+1)
        let instructions = parse_input(input);
        assert_eq!(part2(&instructions), 2);
    }

    #[test]
    fn test_counting() {
        let mut val = Value::new();

        // 50 + 1000  => 50 (passed zero 1050 / 100  = 10)
        assert_eq!(val.execute(&Instruction {
            dir: Direction::Right,
            count: 1000
        }), 10);

        // 50 - 650 = -600 => 0 (passed zero 600 / 100 = 6) + 1
        assert_eq!(val.execute(&Instruction {
            dir: Direction::Left,
            count: 650
        }), 7);

        // 35 + 765 = 800
        let mut val = Value::new_test(35);
        assert_eq!(val.execute(&Instruction { dir: Direction::Right, count: 765 }), 8);
    }
}