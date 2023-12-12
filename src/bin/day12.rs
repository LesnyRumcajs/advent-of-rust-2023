use std::io::BufRead;

use itertools::Itertools;

fn main() {
    let input = read_input(std::io::stdin().lock());
    println!("Day 12, part 1: {}", part1(&input));
    println!("Day 12, part 2: {}", part2(&input));
}

fn part1(input: &[String]) -> i32 {
    let n = input
        .iter()
        .map(|l| l.chars().filter(|&c| c == '?').count())
        .max()
        .unwrap()
        + 1;
    let characters = [".", "#"];

    let mut all_combinations = Vec::new();
    for i in 2..n {
        let combinations: Vec<_> = (2..i).fold(
            characters
                .iter()
                .cartesian_product(characters.iter())
                .map(|(&a, &b)| a.to_owned() + b)
                .collect(),
            |acc, _| {
                acc.into_iter()
                    .cartesian_product(characters.iter())
                    .map(|(a, b)| a.to_owned() + b)
                    .collect()
            },
        );

        all_combinations.push(combinations);
    }

    let mut proper_combinations = 0;
    input.iter().for_each(|l| {
        let (records, arrangements) = l.split_once(' ').unwrap();
        let arrangements = arrangements
            .split(',')
            .map(|c| c.parse::<i32>().unwrap())
            .collect_vec();
        let n = l.chars().filter(|&c| c == '?').count();
        let combinations = &all_combinations[n - 2];

        combinations.iter().for_each(|combination| {
            // replace ? with characters in the combination
            let mut counter = 0;
            let s = records
                .chars()
                .map(|ch| {
                    if ch == '?' {
                        counter += 1;
                        combination.chars().nth(counter - 1).unwrap()
                    } else {
                        ch
                    }
                })
                .collect::<String>();

            let grouped = s
                .split('.')
                .filter(|&s| !s.is_empty())
                .map(|s| s.len() as i32)
                .collect_vec();
            if grouped == arrangements {
                proper_combinations += 1;
            }
        });
    });
    proper_combinations
}
fn part2(_input: &[String]) -> i32 {
    unimplemented!()
}

fn read_input<R: BufRead>(reader: R) -> Vec<String> {
    reader.lines().map(|l| l.unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::File, io::BufReader};

    #[test]
    fn test_solution() {
        let input = BufReader::new(File::open("inputs/day12/input.txt").unwrap());
        let input = read_input(input);

        assert_eq!(6958, part1(&input));
    }

    #[test]
    fn test_example() {
        let input = BufReader::new(File::open("inputs/day12/example.txt").unwrap());
        let input = read_input(input);

        assert_eq!(21, part1(&input));
    }
}
