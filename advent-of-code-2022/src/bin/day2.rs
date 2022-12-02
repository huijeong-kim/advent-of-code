use std::fs::File;
use std::io::BufRead;

#[derive(Debug, Clone, PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissor,
}

impl From<&str> for Shape {
    fn from(val: &str) -> Self {
        match val {
            "A" | "X" => Shape::Rock,
            "B" | "Y" => Shape::Paper,
            "C" | "Z" => Shape:: Scissor,
            _ => panic!("Wrong input")
        }
    }
}

impl Shape {
    fn is_win(me: &Shape, opp: &Shape) -> bool {
        match me {
            Shape::Rock => (opp == &Shape::Scissor),
            Shape::Paper => (opp == &Shape::Rock),
            Shape::Scissor => (opp == &Shape::Paper),
        }
    }

    fn is_draw(me: &Shape, opp: &Shape) -> bool {
        me == opp
    }

    fn get_match_score(&self, opp: &Shape) -> i32 {
        if Shape::is_draw(&self, opp) {
            3
        } else if Shape::is_win(&self, opp) {
            6
        } else {
            0
        }
    }

    fn get_my_score(&self) -> i32 {
        match &self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissor => 3,
        }
    }

    fn what_to_win(&self) -> Shape {
        match &self {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissor,
            Shape::Scissor => Shape::Rock,
        }
    }

    fn what_to_draw(&self) -> Shape {
        self.clone()
    }

    fn what_to_lose(&self) -> Shape {
        match &self {
            Shape::Rock => Shape::Scissor,
            Shape::Paper => Shape::Rock,
            Shape::Scissor => Shape::Paper,
        }
    }

}

enum Result {
    Win,
    Lose,
    Draw,
}

impl From<&str> for Result {
    fn from(val: &str) -> Result {
        match val {
            "X" => Result::Lose,
            "Y" => Result::Draw,
            "Z" => Result::Win,
            _ => panic!("Wrong input"),
        }
    }
}

fn main() {
    let matches = get_day2_input();

    let score :i32 = matches.iter().map(|m| {
        let (opp, me): (Shape, Shape) = (m.0.as_str().into(), m.1.as_str().into());
        me.get_my_score() + me.get_match_score(&opp)
    }).sum();
    println!("{}", score);

    let score: i32 = matches.iter().map(|m| {
        let opp: Shape = m.0.as_str().into();
        let result: Result = m.1.as_str().into();
        let my_decision = match result {
            Result::Win => opp.what_to_win(),
            Result::Draw => opp.what_to_draw(),
            Result::Lose => opp.what_to_lose(),
        };
        my_decision.get_my_score() + my_decision.get_match_score(&opp)
    }).sum();
    println!("{}", score);
}

fn get_day2_input() -> Vec<(String, String)> {
    let file = File::open("inputs/day2.txt").unwrap();
    let lines = std::io::BufReader::new(file).lines();

    let mut result: Vec<(String, String)> = Vec::new();
    for val in lines {
        let line= val.unwrap();
        let transformed: Vec<&str> = line.split_whitespace().collect();
        result.push((transformed[0].to_string(), transformed[1].to_string()));
    }

    result
}

