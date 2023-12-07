use std::{
    collections::HashMap,
    io::{self, BufRead},
};

use itertools::Itertools;

fn main() {
    let stdin = io::stdin().lock().lines().map(|l| l.unwrap()).collect_vec();
    let players = read_input(&stdin, false);
    println!("Day 7, part 1: {}", calculate_winnings(&players));

    let players = read_input(&stdin, true);
    println!("Day 7, part 2: {}", calculate_winnings(&players));
}

fn calculate_winnings(players: &[Player]) -> i32 {
    players
        .iter()
        .sorted()
        .rev()
        .enumerate()
        .map(|(i, p)| p.bid * (i as i32 + 1))
        .sum()
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Hash, Debug, Copy, Clone)]
struct Card(i32);

impl From<char> for Card {
    fn from(c: char) -> Self {
        match c {
            'A' => Card(14),
            'K' => Card(13),
            'Q' => Card(12),
            'J' => Card(11),
            'j' => Card(1),
            'T' => Card(10),
            _ => Card(c.to_digit(10).unwrap() as i32),
        }
    }
}

type Cards = [Card; 5];

#[derive(Debug, PartialEq, Eq)]
struct Player {
    hand: HandResult,
    cards: Cards,
    bid: i32,
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Copy, Clone)]
enum HandResult {
    Five,
    Four,
    FullHouse,
    Three,
    TwoPairs,
    Pair,
    HighCard,
}

impl Ord for Player {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let a = self.hand as i32;
        let b = other.hand as i32;

        match a.cmp(&b) {
            std::cmp::Ordering::Equal => {
                for (a, b) in self.cards.iter().interleave(other.cards.iter()).tuples() {
                    if a != b {
                        return b.cmp(a);
                    }
                }
                panic!("Both players have the same cards");
            }
            _ => a.cmp(&b),
        }
    }
}

impl PartialOrd for Player {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl From<Cards> for HandResult {
    fn from(cards: Cards) -> Self {
        let jokers = cards.iter().filter(|&c| *c == Card(1)).count();
        let mut cards_grouped: HashMap<Card, usize> = cards
            .iter()
            .filter(|&c| c != &Card(1))
            .map(|card| (*card, cards.iter().filter(|c| *c == card).count()))
            .collect();

        if jokers == 5 {
            return HandResult::Five;
        } else if jokers > 0 {
            let max = cards_grouped.values().max().unwrap();
            let max_card = cards_grouped
                .iter()
                .find(|(_, &v)| v == *max)
                .map(|(&k, _)| k)
                .unwrap();

            cards_grouped.insert(max_card, *max + jokers);
        }

        match cards_grouped.values().max().unwrap() {
            5 => HandResult::Five,
            4 => HandResult::Four,
            3 => {
                if cards_grouped.len() == 2 {
                    HandResult::FullHouse
                } else {
                    HandResult::Three
                }
            }
            2 => {
                if cards_grouped.len() == 3 {
                    HandResult::TwoPairs
                } else {
                    HandResult::Pair
                }
            }
            _ => HandResult::HighCard,
        }
    }
}

fn read_input(input: &[String], jokers: bool) -> Vec<Player> {
    input
        .iter()
        .map(|line| {
            let (hand, bid) = line.split_once(' ').unwrap();

            let cards: Cards = hand
                .chars()
                .map(|c| {
                    if jokers && c == 'J' {
                        Card(1)
                    } else {
                        c.into()
                    }
                })
                .collect_vec()
                .try_into()
                .unwrap();

            Player {
                hand: cards.into(),
                cards,
                bid: bid.parse().unwrap(),
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
        let input = BufReader::new(File::open("inputs/day7/input.txt").unwrap())
            .lines()
            .map(|l| l.unwrap())
            .collect_vec();
        let players = read_input(&input, false);
        assert_eq!(248422077, calculate_winnings(&players));
        let players = read_input(&input, true);
        assert_eq!(249817836, calculate_winnings(&players));
    }

    #[test]
    fn test_example() {
        let input = BufReader::new(File::open("inputs/day7/example.txt").unwrap())
            .lines()
            .map(|l| l.unwrap())
            .collect_vec();
        let players = read_input(&input, false);
        assert_eq!(6440, calculate_winnings(&players));
        let players = read_input(&input, true);
        assert_eq!(5905, calculate_winnings(&players));
    }
}
