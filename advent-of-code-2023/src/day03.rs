const TEST_INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

pub fn solution(input: String) {
    let (numbers, symbols) = get(&input);

    let result = part1(&numbers);
    println!("part1: {}", result.iter().sum::<u32>());

    let result = part2(&symbols);
    println!("part2: {:?}", result.iter().sum::<u32>());
}

fn prev_loc(line: usize) -> usize {
    if line == 0 {
        0
    } else {
        line - 1
    }
}
fn next_loc(line: usize) -> usize {
    line + 1
}
fn between(target: usize, min: usize, max: usize) -> bool {
    min <= target && target <= max
}

#[derive(Default, Debug)]
struct Number {
    value: String,
    line: usize,
    start: usize,
    end: usize,
    adjacent_symbols: String,
}
impl Number {
    pub fn new(c: char, loc: (usize, usize)) -> Self {
        Self {
            value: c.to_string(),
            line: loc.0,
            start: loc.1,
            end: loc.1,
            adjacent_symbols: String::new(),
        }
    }
    fn can_be_added(&self, loc: (usize, usize)) -> bool {
        self.line == loc.0 && self.end + 1 == loc.1
    }
    fn add(&mut self, c: char, loc: (usize, usize)) {
        assert_eq!(self.end + 1, loc.1);
        self.end += 1;
        self.value.push(c);
    }

    fn update_adjacent(&mut self, symbols: &mut Vec<Symbol>) {
        for s in symbols.iter_mut() {
            if between(s.loc.0, prev_loc(self.line), next_loc(self.line))
                && between(s.loc.1, prev_loc(self.start), next_loc(self.end))
            {
                self.adjacent_symbols.push(s.value);
                s.adjacent_numbers.push(self.value());
            }
        }
    }

    fn has_adjacent(&self) -> bool {
        self.adjacent_symbols.len() != 0
    }
    fn value(&self) -> u32 {
        self.value.parse().unwrap()
    }
}

#[derive(Debug)]
struct Symbol {
    value: char,
    loc: (usize, usize),
    adjacent_numbers: Vec<u32>,
}

fn get(input: &String) -> (Vec<Number>, Vec<Symbol>) {
    let mut symbols = Vec::new();
    let mut numbers: Vec<Number> = Vec::new();

    input.lines().enumerate().for_each(|(idx, value)| {
        value.chars().enumerate().for_each(|(c_idx, c)| {
            if c.is_numeric() {
                if numbers.last().is_some() && numbers.last().unwrap().can_be_added((idx, c_idx)) {
                    numbers.last_mut().unwrap().add(c, (idx, c_idx));
                } else {
                    numbers.push(Number::new(c, (idx, c_idx)));
                }
            } else if c != '.' {
                symbols.push(Symbol {
                    value: c,
                    loc: (idx, c_idx),
                    adjacent_numbers: Vec::new(),
                });
            }
        })
    });

    numbers.iter_mut().for_each(|num| {
        num.update_adjacent(&mut symbols);
    });

    (numbers, symbols)
}

fn part1(numbers: &Vec<Number>) -> Vec<u32> {
    numbers
        .iter()
        .map(|num| if num.has_adjacent() { num.value() } else { 0 })
        .collect()
}

fn part2(symbols: &Vec<Symbol>) -> Vec<u32> {
    symbols
        .iter()
        .filter(|s| s.value == '*' && s.adjacent_numbers.len() == 2)
        .map(|g| g.adjacent_numbers.iter().fold(1, |acc, &n| acc * n))
        .collect()
}
