use std::fs::File;
use std::io::BufRead;
use std::path::PathBuf;

fn main() {
    let test_commands = vec![
        Command::Forward(5),
        Command::Down(5),
        Command::Forward(8),
        Command::Up(3),
        Command::Down(8),
        Command::Forward(2),
    ];
    let position = part2(&test_commands);
    position.print();

    let commands = get_day2_input();
    let position = part1(&commands);
    position.print();

    let position = part2(&commands);
    position.print();
}

#[derive(Default)]
struct Position {
    x: i32,
    depth: i32,
    aim: i32,
}

impl Position {
    fn simple_execute(&mut self, command: &Command) {
        match command {
            Command::Forward(v) => self.x += v,
            Command::Down(v) => self.depth += v,
            Command::Up(v) => self.depth -= v,
        };
    }

    fn execute(&mut self, command: &Command) {
        match command {
            Command::Forward(v) => {
                self.x += v;
                self.depth += v * self.aim;
            }
            Command::Down(v) => self.aim += v,
            Command::Up(v) => self.aim -= v,
        }
    }

    fn print(&self) {
        println!(
            "x: {}, depth: {} => result: {}",
            self.x,
            self.depth,
            self.x * self.depth
        );
    }
}

#[derive(Debug)]
enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
}

fn get_day2_input() -> Vec<Command> {
    let input = PathBuf::from("inputs/day2.txt");
    let file = File::open(input).unwrap();
    let lines = std::io::BufReader::new(file).lines();

    lines
        .into_iter()
        .map(|s| {
            let line = s.unwrap();
            let data: Vec<&str> = line.split_ascii_whitespace().collect();
            let val = data[1].parse::<i32>().unwrap();
            match data[0] {
                "forward" => Command::Forward(val),
                "down" => Command::Down(val),
                "up" => Command::Up(val),
                _ => panic!("Wrong input"),
            }
        })
        .collect()
}

fn part1(commands: &Vec<Command>) -> Position {
    let mut position = Position::default();
    for c in commands {
        position.simple_execute(&c);
    }

    position
}

fn part2(commands: &Vec<Command>) -> Position {
    let mut position = Position::default();
    for c in commands {
        position.execute(&c);
    }

    position
}
