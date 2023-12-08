use std::collections::HashMap;
use std::io::{self, BufRead};

use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;

fn main() {
    let input = read_input(io::stdin().lock());
    println!("Day 8, part 1: {}", part1(&input));
    println!("Day 8, part 2: {}", part2(&input));
}

fn part1(game: &Game) -> i64 {
    let mut pos = "AAA";
    let mut steps = 0;
    for c in game.directions.iter().cycle() {
        steps += 1;
        if *c == 'L' {
            pos = game.map[pos].0.as_str();
        } else {
            pos = game.map[pos].1.as_str();
        }
        if pos == "ZZZ" {
            return steps;
        }
    }

    panic!("No solution found");
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(nums: &[i64]) -> i64 {
    nums.iter().fold(1, |a, b| a * b / gcd(a, *b))
}

fn part2(game: &Game) -> i64 {
    let pos = game
        .map
        .iter()
        .filter(|(k, _)| k.contains('A'))
        .map(|(k, _)| k)
        .cloned()
        .collect_vec();

    let positions = pos
        .iter()
        .map(|p| {
            let mut p = p.to_owned();
            game.directions
                .iter()
                .cycle()
                .fold_while(0, |steps, c| {
                    if *c == 'L' {
                        p = game.map[&p].0.clone();
                    } else {
                        p = game.map[&p].1.clone();
                    }

                    if p.ends_with('Z') {
                        Done(steps + 1)
                    } else {
                        Continue(steps + 1)
                    }
                })
                .into_inner()
        })
        .collect_vec();

    lcm(&positions)
}

type Map = HashMap<String, (String, String)>;
struct Game {
    map: Map,
    directions: Vec<char>,
}

fn read_input<R: BufRead>(reader: R) -> Game {
    let mut map = Map::new();
    let mut directions = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        if let Some(parts) = line.split_once('=') {
            let from = parts.0.trim();
            let to = parts
                .1
                .chars()
                .filter(|&c| c.is_alphanumeric() || c == ',')
                .collect::<String>();

            let to = to.split_once(',').unwrap();
            map.insert(from.to_string(), (to.0.to_string(), to.1.to_string()));
        } else if !line.is_empty() {
            directions = line.chars().collect();
        }
    }
    Game { map, directions }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::File, io::BufReader};

    #[test]
    fn test_solution() {
        let input = BufReader::new(File::open("inputs/day8/input.txt").unwrap());
        let input = read_input(input);
        assert_eq!(14893, part1(&input));
        assert_eq!(10241191004509, part2(&input));
    }

    #[test]
    fn test_example() {
        let input = BufReader::new(File::open("inputs/day8/example.txt").unwrap());
        let input = read_input(input);
        assert_eq!(6, part1(&input));
    }

    #[test]
    fn test_example2() {
        let input = BufReader::new(File::open("inputs/day8/example2.txt").unwrap());
        let input = read_input(input);
        assert_eq!(6, part2(&input));
    }
}
