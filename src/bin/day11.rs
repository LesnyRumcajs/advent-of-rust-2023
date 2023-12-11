use std::io::BufRead;

use itertools::Itertools;

fn main() {
    let universe = read_input(std::io::stdin().lock());
    println!("Day 11, part 1: {}", solve(&universe, 2));
    println!("Day 11, part 2: {}", solve(&universe, 1000000));
}

fn solve(universe: &Universe, coefficient: usize) -> usize {
    let galaxies = universe.get_galaxies();
    let (expanded_rows, expanded_cols) = universe.expanded_places();

    galaxies
        .iter()
        .tuple_combinations()
        .fold(0usize, |acc, (p1, p2)| {
            acc + (p1.0.min(p2.0)..p1.0.max(p2.0))
                .map(|r| {
                    if expanded_rows.contains(&r) {
                        coefficient
                    } else {
                        1
                    }
                })
                .sum::<usize>()
                + (p1.1.min(p2.1)..p1.1.max(p2.1))
                    .map(|c| {
                        if expanded_cols.contains(&c) {
                            coefficient
                        } else {
                            1
                        }
                    })
                    .sum::<usize>()
        })
}

// https://stackoverflow.com/a/64499219/4658000
fn transpose2<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

#[derive(Debug, Clone)]
struct Universe(Vec<Vec<char>>);

impl Universe {
    fn expanded_places(&self) -> (Vec<usize>, Vec<usize>) {
        let new_universe = self.0.clone();
        let rows = new_universe
            .iter()
            .enumerate()
            .filter(|(_, r)| r.iter().all(|c| *c == '.'))
            .map(|(i, _)| i)
            .collect_vec();

        let new_universe = transpose2(new_universe);
        let cols = new_universe
            .iter()
            .enumerate()
            .filter(|(_, r)| r.iter().all(|c| *c == '.'))
            .map(|(i, _)| i)
            .collect_vec();

        (rows, cols)
    }

    fn get_galaxies(&self) -> Vec<(usize, usize)> {
        self.0.iter().enumerate().fold(vec![], |mut acc, (i, row)| {
            row.iter().enumerate().for_each(|(j, col)| {
                if *col == '#' {
                    acc.push((i, j));
                }
            });
            acc
        })
    }
}

fn read_input<R: BufRead>(reader: R) -> Universe {
    Universe(
        reader
            .lines()
            .map(|l| l.unwrap().chars().collect_vec())
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::File, io::BufReader};

    #[test]
    fn test_solution() {
        let input = BufReader::new(File::open("inputs/day11/input.txt").unwrap());
        let input = read_input(input);

        assert_eq!(9233514, solve(&input, 2));
        assert_eq!(363293506944, solve(&input, 1000000));
    }

    #[test]
    fn test_example() {
        let input = BufReader::new(File::open("inputs/day11/example.txt").unwrap());
        let input = read_input(input);

        assert_eq!(374, solve(&input, 2));
        assert_eq!(1030, solve(&input, 10));
        assert_eq!(8410, solve(&input, 100));
    }
}
