use std::fs::File;
use std::io::BufRead;

#[derive(Debug)]
struct Section {
    start: i32,
    end: i32,
}
impl From<&str> for Section {
    fn from(val: &str) -> Self {
        let values: Vec<&str> = val.split('-').collect();
        Section {
            start: values[0].parse::<i32>().unwrap(),
            end: values[1].parse::<i32>().unwrap(),
        }
    }
}

impl Section {
    fn fully_contains(&self, opp: &Section) -> bool {
        return if self.start <= opp.start && opp.end <= self.end {
            true
        } else if opp.start <= self.start && self.end <= opp.end {
            true
        } else {
            false
        };
    }

    fn is_overlap(&self, opp: &Section) -> bool {
        return if self.end < opp.start {
            false
        } else if opp.end < self.start {
            false
        } else {
            true
        };
    }
}

fn main() {
    let pairs = get_day4_input();

    let counts = pairs
        .iter()
        .filter(|sections| sections.0.fully_contains(&sections.1))
        .count();

    println!("{}", counts);

    let counts = pairs
        .iter()
        .filter(|sections| sections.0.is_overlap(&sections.1))
        .count();

    println!("{}", counts);
}

fn get_day4_input() -> Vec<(Section, Section)> {
    let file = File::open("inputs/day4.txt").unwrap();
    let lines = std::io::BufReader::new(file).lines();
    lines
        .into_iter()
        .map(|line| {
            let line = line.unwrap();
            let ranges: Vec<&str> = line.split(',').collect();
            let first = Section::from(ranges[0]);
            let second = Section::from(ranges[1]);

            (first, second)
        })
        .collect()
}
