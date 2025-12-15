use advent_of_code_2025::read_from_file;

fn main() -> Result<(), std::io::Error> {
    let input = read_from_file("inputs/day05.txt")?;
    let inputs: Vec<&str> = input.split("\n\n").collect();
    assert_eq!(inputs.len(), 2);

    let mut ingredients = Ingredients::from(&inputs[0]);
    let targets = build_targets(&inputs[1]);

    println!("part1={}", part1(&ingredients, &targets));
    println!("part2={}", part2(&mut ingredients));
    Ok(())
}

fn part1(ingredients: &Ingredients, targets: &Vec<i64>) -> i64{
    targets.iter().filter(|val| {
        ingredients.is_fresh(**val)
    }).count() as i64
}

fn part2(ingredients: &mut Ingredients) -> i64 {
    ingredients.count()
}
#[derive(Default, Debug, Clone)]
struct Range {
    start: i64,
    end: i64,
}

impl Range {
    fn from(input: &str) -> Self {
        let nums = input.split('-').collect::<Vec<&str>>();
        assert_eq!(nums.len(), 2);

        Self {
            start: nums[0].parse().unwrap(),
            end: nums[1].parse().unwrap(),
        }
    }
    fn includes(&self, val: i64) -> bool {
        self.start <= val && val <= self.end
    }
    fn overlaps(&self, range: &Range) -> bool {
        if self.start < range.start {
            range.start <= self.end
        } else {
            range.end >= self.start
        }
    }
    fn merge(&mut self, range: &Range) {
        self.start = std::cmp::min(self.start, range.start);
        self.end = std::cmp::max(self.end, range.end);
    }
}

#[derive(Default, Debug)]
struct Ingredients {
    fresh: Vec<Range>,
}

impl Ingredients {
    fn from(input: &str) -> Self {
        let mut ingredients = Ingredients::default();
        input.lines().for_each(|line| {
            ingredients.add(&Range::from(line))
        });
        ingredients.merge();
        ingredients
    }
    fn is_fresh(&self, val: i64) -> bool {
        self.fresh.iter().filter(|range| {
            range.includes(val)
        }).count() > 0
    }
    fn count(&self) -> i64 {
        self.fresh.iter().map(|range| {
            range.end - range.start + 1
        }).sum()
    }
    fn add(&mut self, range: &Range) {
        let mut pending: Vec<Range> = Vec::new();

        if self.fresh.is_empty() {
            pending.push((*range).clone());
        } else {
            let mut added = false;
            for i in 0..self.fresh.len() {
                if self.fresh[i].overlaps(range) {
                    self.fresh[i].merge(range);
                    added = true;
                    break;
                }
            }
            if added == false {
                pending.push((*range).clone());
            }
        }
        
        self.fresh.append(&mut pending);

        self.fresh.sort_by(|a, b| {
            if a.start == b.start {
                a.end.cmp(&b.end)
            } else {
                a.start.cmp(&b.start)
            }
        });
        println!("{:?}", self.fresh);
    }
    fn merge(&mut self) {
        let before = self.fresh.clone();
        self.fresh.clear();

        before.iter().for_each(|range| {
            self.add(range);
        });
    }
}

fn build_targets(input: &str) -> Vec<i64> {
    input.lines().map(|line| {
        line.parse::<i64>().unwrap()
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
    #[test]
    fn test_part1() {
        let inputs: Vec<&str> = INPUT.split("\n\n").collect();
        assert_eq!(inputs.len(), 2);

        let ingredients = Ingredients::from(&inputs[0]);
        let targets = build_targets(&inputs[1]);
        assert_eq!(part1(&ingredients, &targets), 3);
    }

    #[test]
    fn test_part2() {
        let inputs: Vec<&str> = INPUT.split("\n\n").collect();
        assert_eq!(inputs.len(), 2);

        let mut ingredients = Ingredients::from(&inputs[0]);
        assert_eq!(part2(&mut ingredients), 14);
    }
}