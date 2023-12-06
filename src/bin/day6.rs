use std::io::{self, BufRead};

use itertools::Itertools;

fn main() {
    let input = read_input(io::stdin().lock());
    println!("Day 6, part 1: {}", part1(&input));
    println!("Day 6, part 2: {}", part2(&input));
}

fn part1(races: &[Race]) -> i64 {
    races.iter().fold(1, |acc, race| {
        acc * (1..race.distance).fold(0, |acc, speed| {
            acc + if speed * (race.distance - speed) > race.record {
                1
            } else {
                0
            }
        })
    })
}
fn part2(races: &[Race]) -> i64 {
    let (distance, record) = races
        .iter()
        .fold((String::new(), String::new()), |acc, race| {
            (
                acc.0 + &race.distance.to_string(),
                acc.1 + &race.record.to_string(),
            )
        });
    let race = Race {
        distance: distance.parse().unwrap(),
        record: record.parse().unwrap(),
    };

    (1..race.distance).fold(0, |acc, speed| {
        acc + if speed * (race.distance - speed) > race.record {
            1
        } else {
            0
        }
    })
}

struct Race {
    distance: i64,
    record: i64,
}

fn read_input<R: BufRead>(reader: R) -> Vec<Race> {
    let input = reader
        .lines()
        .map_while(Result::ok)
        .map(|l| {
            l.split_whitespace()
                .skip(1)
                .map(|v| v.parse::<i64>().unwrap())
                .collect_vec()
        })
        .collect_vec();

    input[0]
        .iter()
        .interleave(input[1].iter())
        .tuples()
        .map(|(&distance, &record)| Race { distance, record })
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::File, io::BufReader};

    #[test]
    fn test_solution() {
        let input = BufReader::new(File::open("inputs/day6/input.txt").unwrap());
        let input = read_input(input);
        assert_eq!(316800, part1(&input));
        assert_eq!(45647654, part2(&input));
    }

    #[test]
    fn test_example() {
        let input = BufReader::new(File::open("inputs/day6/example.txt").unwrap());
        let input = read_input(input);
        assert_eq!(288, part1(&input));
        assert_eq!(71503, part2(&input));
    }
}
