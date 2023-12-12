pub fn solution(input: String) {
    let result = part1(&input);
    println!("result = {}", result);

    let result = part2(&input);
    println!("result = {}", result);
}
fn parses_input(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| {
            line.split(' ')
                .map(|number| number.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect()
}

fn get_diff_vectors(numbers: &Vec<i64>) -> Vec<Vec<i64>> {
    let mut diffs = Vec::new();
    diffs.push(numbers.clone());

    while !diffs.last().unwrap().iter().all(|&val| val == 0) {
        let iter = diffs.last().unwrap().iter().as_slice().windows(2);
        let current_diffs = iter.map(|value| value[1] - value[0]).collect::<Vec<_>>();
        diffs.push(current_diffs);
    }

    diffs
}

fn predict_prev(numbers: &Vec<i64>) -> i64 {
    let diffs = get_diff_vectors(numbers);

    diffs
        .into_iter()
        .rev()
        .fold(0, |acc, val| val.first().unwrap() - acc)
}

fn predict_next(numbers: &Vec<i64>) -> i64 {
    let diffs = get_diff_vectors(numbers);

    diffs
        .into_iter()
        .rev()
        .fold(0, |acc, val| acc + val.last().unwrap())
}

fn part1(input: &str) -> i64 {
    let numbers = parses_input(&input);

    numbers
        .iter()
        .map(|numbers| predict_next(numbers))
        .collect::<Vec<_>>()
        .iter()
        .sum()
}

fn part2(input: &str) -> i64 {
    let numbers = parses_input(&input);

    numbers
        .iter()
        .map(|numbers| predict_prev(numbers))
        .collect::<Vec<_>>()
        .iter()
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    const TEST_INPUT_LONG: &str = "6 10 30 80 170 314 561 1060 2170 4641 9947 20986 43640 90222 186816 388245 807350 1670120 3415956 6874320 13566998";

    #[test]
    fn test_parse_input() {
        let actual = parses_input(TEST_INPUT);

        let expected: Vec<Vec<i64>> = vec![
            vec![0, 3, 6, 9, 12, 15],
            vec![1, 3, 6, 10, 15, 21],
            vec![10, 13, 16, 21, 30, 45],
        ];

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_predict_next() {
        let vec = vec![0, 3, 6, 9, 12, 15];
        assert_eq!(predict_next(&vec), 18);

        let vec = vec![1, 3, 6, 10, 15, 21];
        assert_eq!(predict_next(&vec), 28);

        let vec = vec![10, 13, 16, 21, 30, 45];
        assert_eq!(predict_next(&vec), 68);
    }

    #[test]
    fn test_predict_prev() {
        let vec = vec![0, 3, 6, 9, 12, 15];
        assert_eq!(predict_prev(&vec), -3);

        let vec = vec![1, 3, 6, 10, 15, 21];
        assert_eq!(predict_prev(&vec), 0);

        let vec = vec![10, 13, 16, 21, 30, 45];
        assert_eq!(predict_prev(&vec), 5);
    }

    #[test]
    fn test_part1_with_test_input() {
        let result = part1(TEST_INPUT);

        assert_eq!(result, 114);
    }

    #[test]
    fn test_part1() {
        let input = crate::read_from_file("inputs/day09.txt");
        let result = part1(&input);

        assert_eq!(result, 2043183816);
    }

    #[test]
    fn test_part2_with_test_input() {
        let result = part2(TEST_INPUT);

        assert_eq!(result, 2);
    }

    #[test]
    fn test_with_target_input() {
        let result = part1(TEST_INPUT_LONG);
        println!("{}", result);
    }

    #[test]
    fn test_part2() {
        let input = crate::read_from_file("inputs/day09.txt");
        let result = part2(&input);

        assert_eq!(result, 1118);
    }
}
