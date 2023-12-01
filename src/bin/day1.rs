use std::io::{self, BufRead};

fn main() {
    let numbers = read_input(io::stdin().lock());
    println!("Day 1, part 1: {}", part1(&numbers));
    println!("Day 1, part 2: {}", part2(&numbers));
}

fn part1(nums: &[String]) -> u32 {
    nums.iter()
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<_>>()
        })
        .fold(0, |acc, x| {
            let num = 10 * x.first().unwrap() + x.last().unwrap();
            acc + num
        })
}

fn part2(nums: &[String]) -> u32 {
    nums.iter()
        .map(|line| {
            line.replace("one", "o1e")
                .replace("two", "t2o")
                .replace("three", "t3e")
                .replace("four", "f4r")
                .replace("five", "f5e")
                .replace("six", "s6x")
                .replace("seven", "s7n")
                .replace("eight", "e8t")
                .replace("nine", "n9e")
                .chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<_>>()
        })
        .fold(0, |acc, x| {
            let num = 10 * x[0] + x[x.len() - 1];
            acc + num
        })
}

fn read_input<R: BufRead>(reader: R) -> Vec<String> {
    reader.lines().map_while(Result::ok).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::File, io::BufReader};

    #[test]
    fn test_solution() {
        let input = BufReader::new(File::open("inputs/day1/input.txt").unwrap());
        let input = read_input(input);
        assert_eq!(55712, part1(&input));
        assert_eq!(55413, part2(&input));
    }

    #[test]
    fn test_example() {
        let input = BufReader::new(File::open("inputs/day1/example.txt").unwrap());
        let input = read_input(input);
        assert_eq!(142, part1(&input));
    }

    #[test]
    fn test_example2() {
        let input = BufReader::new(File::open("inputs/day1/example2.txt").unwrap());
        let input = read_input(input);
        assert_eq!(281, part2(&input));
    }
}
