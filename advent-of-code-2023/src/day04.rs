const TEST_INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

pub fn solution(input: String) {
    let mut cards = get(&input);

    let result = part1(&cards);
    println!("part1: {:?}", result.iter().sum::<i32>());

    let result = part2(&mut cards);
    println!("part2: {:?}", result.iter().sum::<i32>());
}

struct Card {
    winning: Vec<i32>,
    numbers: Vec<i32>,
    matches: i32,
    copies: i32,
}
fn get(input: &String) -> Vec<Card> {
    input
        .lines()
        .map(|l| {
            let split: Vec<_> = l.split(":").collect();
            let only_numbers: Vec<_> = split[1].split("|").collect();

            let winning: Vec<_> = only_numbers[0]
                .split(" ")
                .filter(|n| *n != "")
                .map(|n| n.parse::<i32>().unwrap())
                .collect();
            let numbers: Vec<_> = only_numbers[1]
                .split(" ")
                .filter(|n| *n != "")
                .map(|n| n.parse::<i32>().unwrap())
                .collect();

            let matches: i32 = numbers
                .iter()
                .map(|n| if winning.contains(n) { 1 } else { 0 })
                .sum();

            Card {
                winning,
                numbers,
                matches,
                copies: 1,
            }
        })
        .collect()
}
fn part1(cards: &Vec<Card>) -> Vec<i32> {
    cards
        .iter()
        .map(|c| {
            if c.matches == 0 {
                0
            } else {
                let base = 2i32;
                base.pow(c.matches as u32 - 1)
            }
        })
        .collect()
}

fn part2(cards: &mut Vec<Card>) -> Vec<i32> {
    for idx in 0..cards.len() {
        let card = &cards[idx];
        let num_cards_to_update = card.matches;
        let num_copies = card.copies;

        for i in 1..=num_cards_to_update as usize {
            if idx + 1 < cards.len() {
                cards[idx + i].copies += num_copies;
            }
        }
    }

    cards.iter().map(|c| c.copies).collect()
}
