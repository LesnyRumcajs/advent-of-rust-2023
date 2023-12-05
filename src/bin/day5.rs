use itertools::Itertools;
use rayon::prelude::*;
use std::{
    collections::HashMap,
    io::{self, BufRead},
    ops::Range,
    sync::atomic::AtomicI64,
};

fn main() {
    let (seeds, maps) = read_input(io::stdin().lock());
    println!("Day 5, part 1: {}", part1(&seeds, &maps));
    println!("Day 5, part 2: {}", part2(&seeds, &maps));
}

fn part1(seeds: &Seeds, maps: &Maps) -> i64 {
    let mut locations = Vec::new();
    for seed in seeds {
        let mut location = *seed;
        for map in maps {
            for (src_range, dest_range) in map {
                if src_range.contains(&location) {
                    let diff = location - src_range.start;
                    location = dest_range.start + diff;
                    break;
                }
            }
        }

        locations.push(location);
    }

    *locations.iter().min().unwrap()
}

fn part2(seeds: &Seeds, maps: &Maps) -> i64 {
    let seeds = seeds
        .chunks(2)
        .map(|chunk| chunk[0]..chunk[0] + chunk[1])
        .collect_vec();

    seeds
        .par_iter()
        .map(|seed_range| {
            let min_location = AtomicI64::new(i64::MAX);
            seed_range.clone().into_par_iter().for_each(|seed| {
                let mut location = seed;
                for map in maps {
                    for (src_range, dest_range) in map {
                        if src_range.contains(&location) {
                            let diff = location - src_range.start;
                            location = dest_range.start + diff;
                            break;
                        }
                    }
                }

                min_location.fetch_min(location, std::sync::atomic::Ordering::Relaxed);
            });
            min_location.into_inner()
        })
        .min()
        .unwrap()
}

type Seeds = Vec<i64>;
type Maps = Vec<HashMap<Range<i64>, Range<i64>>>;

fn read_input<R: BufRead>(reader: R) -> (Seeds, Maps) {
    let mut seeds = None;
    let mut maps = Vec::new();
    let mut current_map = HashMap::new();
    for line in reader.lines() {
        let line = line.unwrap();

        if line.is_empty() {
            continue;
        }

        if line.starts_with("seeds") {
            seeds = Some(
                line.split_whitespace()
                    .skip(1)
                    .map(|s| s.parse::<i64>().unwrap())
                    .collect::<Vec<_>>(),
            );

            continue;
        }

        if line.ends_with("map:") {
            if !current_map.is_empty() {
                maps.push(current_map);
            }
            current_map = HashMap::new();
            continue;
        }

        if line.chars().next().unwrap().is_ascii_digit() {
            let (dest_start, src_start, len) = {
                let mut parts = line.split_whitespace();
                let dest_start = parts.next().unwrap().parse::<i64>().unwrap();
                let src_start = parts.next().unwrap().parse::<i64>().unwrap();
                let len = parts.next().unwrap().parse::<i64>().unwrap();
                (dest_start, src_start, len)
            };

            let dest_range = dest_start..dest_start + len;
            let src_range = src_start..src_start + len;
            current_map.insert(src_range, dest_range);
        }
    }
    maps.push(current_map);

    (seeds.unwrap(), maps)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::File, io::BufReader};

    #[test]
    fn test_solution() {
        let input = BufReader::new(File::open("inputs/day5/input.txt").unwrap());
        let (seeds, maps) = read_input(input);
        assert_eq!(227653707, part1(&seeds, &maps));
        // Too long on for GH worker, short enough on my machine with 32 cores :)
        // assert_eq!(78775051, part2(&seeds, &maps));
    }

    #[test]
    fn test_example() {
        let input = BufReader::new(File::open("inputs/day5/example.txt").unwrap());
        let (seeds, maps) = read_input(input);
        assert_eq!(35, part1(&seeds, &maps));
        assert_eq!(46, part2(&seeds, &maps));
    }
}
