use std::{
    collections::HashMap,
    io::{self, BufRead},
};

fn main() {
    let input = read_input(io::stdin().lock());
    println!("Day 2, part 1: {}", part1(&input));
    println!("Day 2, part 2: {}", part2(&input));
}

fn part1(cards: &[Card]) -> i32 {
    cards.iter().fold(0, |acc, card| {
        let result = card.have.iter().fold(0, |acc, have| {
            if card.winning.contains(have) {
                acc + 1
            } else {
                acc
            }
        });

        acc + if result > 0 { 1 << (result - 1) } else { 0 }
    })
}

fn part2(cards: &[Card]) -> i32 {
    let mut cards_count: HashMap<i32, i32> =
        HashMap::from_iter(cards.iter().map(|card| (card.id, 1)));
    cards.iter().for_each(|card| {
        let result = card.have.iter().fold(0, |acc, have| {
            if card.winning.contains(have) {
                acc + 1
            } else {
                acc
            }
        });

        if result > 0 {
            for id in (card.id + 1)..=(card.id + result) {
                *cards_count.entry(id).or_insert(0) += cards_count[&card.id];
            }
        }
    });

    cards_count.values().sum()
}

struct Card {
    id: i32,
    winning: Vec<i32>,
    have: Vec<i32>,
}

fn read_input<R: BufRead>(reader: R) -> Vec<Card> {
    reader
        .lines()
        .map_while(Result::ok)
        .map(|l| {
            let (card_name, cards) = l.split_once(": ").unwrap();
            let card_id = card_name
                .split_whitespace()
                .nth(1)
                .unwrap()
                .parse::<i32>()
                .unwrap();

            let cards = cards.split_once(" | ").unwrap();

            let winning = cards
                .0
                .split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect();
            let have = cards
                .1
                .split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect();

            Card {
                id: card_id,
                winning,
                have,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::File, io::BufReader};

    #[test]
    fn test_solution() {
        let input = BufReader::new(File::open("inputs/day4/input.txt").unwrap());
        let input = read_input(input);
        assert_eq!(21213, part1(&input));
        assert_eq!(8549735, part2(&input));
    }

    #[test]
    fn test_example_1() {
        let input = BufReader::new(File::open("inputs/day4/example.txt").unwrap());
        let input = read_input(input);
        assert_eq!(13, part1(&input));
    }

    #[test]
    fn test_example_2() {
        let input = BufReader::new(File::open("inputs/day4/example.txt").unwrap());
        let input = read_input(input);
        assert_eq!(30, part2(&input));
    }
}
