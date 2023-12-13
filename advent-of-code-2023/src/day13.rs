pub fn solution(input: String) {
    let result = part1(&input);
    println!("part1: {}", result);

    let result = part2(&input);
    println!("part2: {}", result);
}

fn parse_patterns(input: &str) -> Vec<String> {
    let mut patterns = Vec::new();
    let mut current = String::new();
    input.lines().for_each(|line| {
        if !line.is_empty() {
            current.push_str(line);
            current.push_str("\n");
        } else {
            patterns.push(current.clone());
            current = String::new();
        }
    });
    if !current.is_empty() {
        patterns.push(current.clone());
    }

    patterns
}
fn part1(input: &str) -> u64 {
    let patterns = parse_patterns(input);
    patterns.iter().map(|p| value(p, 0)).sum()
}

fn part2(input: &str) -> u64 {
    let patterns = parse_patterns(input);
    patterns.iter().map(|p| value(p, 1)).sum()
}

fn value(pattern: &str, target_diff: u64) -> u64 {
    let pattern = pattern.lines().collect::<Vec<_>>();
    if let Some(row) = find_horizontal_reflection(&pattern, target_diff) {
        return 100 * row;
    } else if let Some(col) = find_vertical_reflection(&pattern, target_diff) {
        return col;
    } else {
        unreachable!();
    }
}

fn find_horizontal_reflection(pattern: &Vec<&str>, target_diff: u64) -> Option<u64> {
    for idx in 0..pattern.len() - 1 {
        if difference_in_horizontally_mirror(pattern, idx) == target_diff {
            return Some(idx as u64 + 1);
        }
    }

    None
}

fn difference_in_horizontally_mirror(pattern: &Vec<&str>, idx: usize) -> u64 {
    let length = std::cmp::min(idx + 1, pattern.len() - idx - 1);
    let mut difference_counter = 0;
    for offset in 0..length {
        let up = pattern[idx - offset];
        let down = pattern[idx + offset + 1];

        for char_idx in 0..up.len() {
            if up.chars().nth(char_idx) != down.chars().nth(char_idx) {
                difference_counter += 1;
            }
        }
    }

    return difference_counter;
}

fn find_vertical_reflection(pattern: &Vec<&str>, target_diff: u64) -> Option<u64> {
    let total_length = pattern[0].len();
    for idx in 0..total_length - 1 {
        if difference_in_vertical_mirror(pattern, idx) == target_diff {
            return Some(idx as u64 + 1);
        }
    }
    None
}

fn difference_in_vertical_mirror(pattern: &Vec<&str>, idx: usize) -> u64 {
    let total_length = pattern[0].len();
    let length = std::cmp::min(idx + 1, total_length - idx - 1);
    let mut difference_counter = 0;

    for offset in 0..length {
        let differences = pattern
            .iter()
            .filter(|&line| {
                let left = line.chars().nth(idx - offset).unwrap();
                let right = line.chars().nth(idx + offset + 1).unwrap();

                left != right
            })
            .count() as u64;
        difference_counter += differences;
    }

    return difference_counter;
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn test_part1_with_test_input() {
        let result = part1(TEST_INPUT);
        assert_eq!(result, 405);
    }

    #[test]
    fn test_part2_with_test_input() {
        let result = part2(TEST_INPUT);
        assert_eq!(result, 400);
    }

    #[test]
    fn test_find_horizontal_mirror() {
        let input = "#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"
            .lines()
            .collect::<Vec<_>>();
        let result = find_horizontal_reflection(&input, 0);
        assert_eq!(result, Some(4));
        let result = find_vertical_reflection(&input, 0);
        assert_eq!(result, None);
    }

    #[test]
    fn test_find_vertical_mirror() {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#."
            .lines()
            .collect::<Vec<_>>();
        let result = find_horizontal_reflection(&input, 0);
        assert_eq!(result, None);
        let result = find_vertical_reflection(&input, 0);
        assert_eq!(result, Some(5));
    }
}
