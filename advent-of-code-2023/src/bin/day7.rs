use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(PartialEq, Debug, PartialOrd, Ord, Eq)]
enum HandType {
    FiveKind,
    FourKind,
    FullHouse,
    ThreeKind,
    TwoPair,
    OnePair,
    HighCard,
    None,
}

const CARD_RANK: [char; 13] = [
    'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
];
const CARD_RANK_J: [char; 13] = [
    'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
];

struct Hand {
    cards: String,
    bid: u64,
    hand_type: HandType,
    rank: [char; 13],
}

impl Hand {
    fn card_to_rank(&self, c: &char) -> u64 {
        self.rank
            .iter()
            .enumerate()
            .filter(|(_, v)| *v == c)
            .map(|(idx, _)| idx as u64)
            .collect::<Vec<_>>()[0]
    }
}

impl Eq for Hand {}

impl PartialEq<Self> for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards && self.bid == other.bid
    }
}

impl PartialOrd<Self> for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        return if self.hand_type != other.hand_type {
            self.hand_type.cmp(&other.hand_type)
        } else {
            for (idx, c) in self.cards.chars().enumerate() {
                let other_char = other.cards.chars().nth(idx).unwrap();

                if c != other_char {
                    let self_rank = self.card_to_rank(&c);
                    let other_rank = self.card_to_rank(&other_char);

                    return self_rank.cmp(&other_rank);
                }
            }

            Ordering::Equal
        };
    }
}

impl Debug for Hand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {:?}", self.cards, self.hand_type)
    }
}

fn cards_to_counts(cards: &str) -> HashMap<char, i32> {
    let mut kinds = HashMap::new();
    cards.chars().for_each(|c| {
        if let Some(value) = kinds.get_mut(&c) {
            *value = *value + 1;
        } else {
            kinds.insert(c, 1);
        }
    });

    kinds
}

fn values_to_type(values: &Vec<i32>) -> HandType {
    return if values.len() == 0 {
        HandType::None
    } else if values.contains(&5) {
        HandType::FiveKind
    } else if values.contains(&4) {
        HandType::FourKind
    } else if values.contains(&3) && values.contains(&2) {
        HandType::FullHouse
    } else if values.contains(&3) {
        HandType::ThreeKind
    } else if values.iter().filter(|&&x| x == 2).count() == 2 {
        HandType::TwoPair
    } else if values.iter().filter(|&&x| x == 2).count() == 1 {
        HandType::OnePair
    } else if values.iter().filter(|&&x| x == 1).count() == values.len() {
        HandType::HighCard
    } else {
        panic!("can't reach here");
    };
}

fn to_hand_type(cards: &str) -> HandType {
    let kinds = cards_to_counts(cards);
    let values: Vec<_> = kinds.iter().map(|v| *v.1).collect();
    values_to_type(&values)
}

fn to_hand_type_with_joker(cards: &str) -> HandType {
    let kinds = cards_to_counts(cards);

    let mut values: Vec<_> = kinds.iter().filter(|v| *v.0 != 'J').map(|v| *v.1).collect();
    let current_type = values_to_type(&mut values);

    return if let Some(&num_jokers) = kinds.get(&'J') {
        assert!(num_jokers > 0);
        let new_type = match current_type {
            HandType::FiveKind => {
                panic!("can't reach here, {}", cards);
            }
            HandType::FourKind => HandType::FiveKind,
            HandType::FullHouse => {
                panic!("can't reach here, {}", cards);
            }
            HandType::ThreeKind => {
                if num_jokers == 1 {
                    HandType::FourKind
                } else if num_jokers == 2 {
                    HandType::FiveKind
                } else {
                    panic!("can't reach here, {}", cards);
                }
            }
            HandType::TwoPair => HandType::FullHouse,
            HandType::OnePair => {
                if num_jokers == 1 {
                    HandType::ThreeKind
                } else if num_jokers == 2 {
                    HandType::FourKind
                } else if num_jokers == 3 {
                    HandType::FiveKind
                } else {
                    panic!("can't reach here, {}", cards);
                }
            }
            HandType::HighCard => {
                if num_jokers == 1 {
                    HandType::OnePair
                } else if num_jokers == 2 {
                    HandType::ThreeKind
                } else if num_jokers == 3 {
                    HandType::FourKind
                } else if num_jokers == 4 {
                    HandType::FiveKind
                } else {
                    panic!("can't reach here, {}", cards);
                }
            }
            HandType::None => {
                assert_eq!(num_jokers, 5);
                HandType::FiveKind
            }
        };

        new_type
    } else {
        current_type
    };
}

fn day7_input() -> Vec<String> {
    let file = File::open("inputs/day7.txt").unwrap();
    let buf_reader = BufReader::new(file);
    buf_reader.lines().map(|l| l.unwrap()).collect()
}

fn main() {
    let input = day7_input();
    let result = part1(&input);
    println!("result: {}", result);

    let result = part2(&input);
    println!("result: {}", result);
}

fn part1(input: &Vec<String>) -> u64 {
    let mut hands: Vec<_> = input
        .iter()
        .map(|line| {
            let values: Vec<_> = line.split(" ").collect();
            assert_eq!(values.len(), 2);

            Hand {
                cards: values[0].to_string(),
                bid: values[1].parse::<u64>().unwrap(),
                hand_type: to_hand_type(values[0]),
                rank: CARD_RANK,
            }
        })
        .collect();

    hands.sort_by(|a, b| b.cmp(a));

    let mut rank = 1;
    let mut result = 0;
    for hand in hands.iter() {
        let value = hand.bid * rank;
        result += value;
        rank += 1;
    }

    result
}

fn part2(input: &Vec<String>) -> u64 {
    let mut hands: Vec<_> = input
        .iter()
        .map(|line| {
            let values: Vec<_> = line.split(" ").collect();
            assert_eq!(values.len(), 2);

            Hand {
                cards: values[0].to_string(),
                bid: values[1].parse::<u64>().unwrap(),
                hand_type: to_hand_type_with_joker(values[0]),
                rank: CARD_RANK_J,
            }
        })
        .collect();

    hands.sort_by(|a, b| b.cmp(a));

    let mut rank = 1;
    let mut result = 0;
    for hand in hands.iter() {
        result += hand.bid * rank;
        rank += 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_input() -> Vec<String> {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        input.lines().map(|l| l.to_string()).collect()
    }

    #[test]
    fn test_find_hand_type() {
        assert_eq!(to_hand_type("AAAAA"), HandType::FiveKind);
        assert_eq!(to_hand_type("AA8AA"), HandType::FourKind);
        assert_eq!(to_hand_type("23332"), HandType::FullHouse);
        assert_eq!(to_hand_type("TTT98"), HandType::ThreeKind);
        assert_eq!(to_hand_type("23432"), HandType::TwoPair);
        assert_eq!(to_hand_type("A23A4"), HandType::OnePair);
        assert_eq!(to_hand_type("23456"), HandType::HighCard);
    }

    #[test]
    fn test_hand_type_compare() {
        assert!(HandType::FiveKind < HandType::FourKind);
        assert!(HandType::FourKind < HandType::FullHouse);
        assert!(HandType::FullHouse < HandType::ThreeKind);
        assert!(HandType::ThreeKind < HandType::TwoPair);
        assert!(HandType::TwoPair < HandType::OnePair);
        assert!(HandType::OnePair < HandType::HighCard);
    }

    fn create_hand(cards: &str) -> Hand {
        Hand {
            cards: cards.to_string(),
            bid: 0,
            hand_type: to_hand_type(cards),
            rank: CARD_RANK,
        }
    }

    #[test]
    fn test_hand_compare() {
        let hand_a = create_hand("33332");
        let hand_b = create_hand("2AAAA");
        assert!(hand_a < hand_b);

        let hand_a = create_hand("77888");
        let hand_b = create_hand("77788");
        assert!(hand_a < hand_b);
    }

    #[test]
    fn test_part1_with_test_input() {
        let input = test_input();
        let result = part1(&input);

        assert_eq!(result, 6440);
    }

    #[test]
    fn test_part1() {
        let input = day7_input();
        let result = part1(&input);

        assert_eq!(result, 241344943);
    }

    #[test]
    fn test_part2_with_test_input() {
        let input = test_input();
        let result = part2(&input);

        assert_eq!(result, 5905);
    }

    #[test]
    fn test_part2() {
        let input = day7_input();
        let result = part2(&input);

        assert_eq!(result, 243101568);
    }

    #[test]
    fn test_hand_type_with_joker() {
        assert_eq!(to_hand_type_with_joker("32T3K"), HandType::OnePair);
        assert_eq!(to_hand_type_with_joker("T55J5"), HandType::FourKind);
        assert_eq!(to_hand_type_with_joker("KK677"), HandType::TwoPair);
        assert_eq!(to_hand_type_with_joker("KTJJT"), HandType::FourKind);
        assert_eq!(to_hand_type_with_joker("QQQJA"), HandType::FourKind);
    }
}
