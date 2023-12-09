use std::io::{self, BufRead};

use itertools::Itertools;

fn main() {
    let input = read_input(io::stdin().lock());
    let (part2, part1) = interpolate(&input);
    println!("Day 9, part 1: {part1}");
    println!("Day 9, part 2: {part2}");
}

fn interpolate(input: &[Vec<i32>]) -> (i32, i32) {
    input.iter().fold((0, 0), |acc, v| {
        let mut diffs = vec![v.clone()];
        let mut curr_diff = v.clone();

        while !curr_diff.is_empty() && curr_diff.iter().any(|v| *v != 0) {
            let new_diff = curr_diff
                .iter()
                .tuple_windows()
                .map(|(a, b)| b - a)
                .collect_vec();
            diffs.push(new_diff.clone());
            curr_diff = new_diff;
        }

        let mut new_first = 0;
        diffs.iter().skip(1).rev().for_each(|v| {
            new_first = v[0] - new_first;
        });
        let post = diffs.iter().map(|v| v.last().unwrap()).sum::<i32>();
        (acc.0 + diffs[0][0] - new_first, acc.1 + post)
    })
}

fn read_input<R: BufRead>(reader: R) -> Vec<Vec<i32>> {
    reader
        .lines()
        .map(|l| {
            l.unwrap()
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::File, io::BufReader};

    #[test]
    fn test_solution() {
        let input = BufReader::new(File::open("inputs/day9/input.txt").unwrap());
        let input = read_input(input);

        let (part2, part1) = interpolate(&input);
        assert_eq!(1938800261, part1);
        assert_eq!(1112, part2);
    }

    #[test]
    fn test_example() {
        let input = BufReader::new(File::open("inputs/day9/example.txt").unwrap());
        let input = read_input(input);

        let (part2, part1) = interpolate(&input);
        assert_eq!(114, part1);
        assert_eq!(2, part2);
    }
}
