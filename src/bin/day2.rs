use std::{
    io::{self, BufRead},
    str::FromStr,
};

fn main() {
    let input = read_input(io::stdin().lock());
    println!("Day 2, part 1: {}", part1(&input));
    println!("Day 2, part 2: {}", part2(&input));
}

fn part1(games: &[Game]) -> i32 {
    games
        .iter()
        .filter(|&game| {
            !game
                .sets
                .iter()
                .any(|set| set.reds > 12 || set.greens > 13 || set.blues > 14)
        })
        .fold(0, |acc, game| acc + game.id)
}

fn part2(games: &[Game]) -> i32 {
    games
        .iter()
        .map(|game| {
            game.sets
                .iter()
                .fold(CubeSet::default(), |acc, set| CubeSet {
                    blues: acc.blues.max(set.blues),
                    reds: acc.reds.max(set.reds),
                    greens: acc.greens.max(set.greens),
                })
        })
        .fold(0, |acc, set| acc + set.blues * set.reds * set.greens)
}

#[derive(Default)]
struct CubeSet {
    blues: i32,
    reds: i32,
    greens: i32,
}

impl FromStr for CubeSet {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = regex::Regex::new(r"(\d+) (r|g|b)").unwrap();
        let mut set = CubeSet::default();

        for cap in re.captures_iter(s) {
            let num = cap[1].parse::<i32>().unwrap();
            match &cap[2] {
                "r" => set.reds = num,
                "g" => set.greens = num,
                "b" => set.blues = num,
                _ => panic!("Invalid input"),
            }
        }

        Ok(set)
    }
}

struct Game {
    id: i32,
    sets: Vec<CubeSet>,
}

fn read_input<R: BufRead>(reader: R) -> Vec<Game> {
    reader
        .lines()
        .map_while(Result::ok)
        .map(|l| {
            let (game, sets) = l.split_once(": ").unwrap();
            let game_id = game.split_once(' ').unwrap().1.parse::<i32>().unwrap();

            let sets = sets
                .split("; ")
                .map(|s| s.parse::<CubeSet>().unwrap())
                .collect::<Vec<_>>();

            Game { id: game_id, sets }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::File, io::BufReader};

    #[test]
    fn test_solution() {
        let input = BufReader::new(File::open("inputs/day2/input.txt").unwrap());
        let input = read_input(input);
        assert_eq!(2237, part1(&input));
        assert_eq!(66681, part2(&input));
    }

    #[test]
    fn test_example_1() {
        let input = BufReader::new(File::open("inputs/day2/example1.txt").unwrap());
        let input = read_input(input);
        assert_eq!(8, part1(&input));
    }

    #[test]
    fn test_example_2() {
        let input = BufReader::new(File::open("inputs/day2/example1.txt").unwrap());
        let input = read_input(input);
        assert_eq!(2286, part2(&input));
    }
}
