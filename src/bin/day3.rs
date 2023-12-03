use std::io::{self, BufRead};

fn main() {
    let input = read_input(io::stdin().lock());
    println!("Day 3, part 1: {}", part1(&input));
    println!("Day 3, part 2: {}", part2(&input));
}

fn part1(input: &[String]) -> i32 {
    let mut numbers = Vec::new();
    let mut is_adjacent = false;
    let mut number = 0;
    input.iter().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            if !c.is_ascii_digit() {
                return;
            }

            number = number * 10 + c.to_digit(10).unwrap() as i32;

            let neighbours = get_neighbours(x, y, input);
            is_adjacent =
                is_adjacent || neighbours.iter().any(|&n| n != '.' && !n.is_ascii_digit());

            if x == input[y].len() - 1 || !input[y].chars().nth(x + 1).unwrap().is_ascii_digit() {
                if is_adjacent {
                    numbers.push(number);
                }
                is_adjacent = false;
                number = 0;
            }
        })
    });

    numbers.iter().sum()
}
fn part2(input: &[String]) -> i32 {
    input.iter().enumerate().fold(0, |acc, (y, line)| {
        acc + line
            .chars()
            .enumerate()
            .filter(|c| c.1 == '*')
            .fold(0, |acc, (x, _)| {
                let neighbours = get_neighbour_numbers(x, y, input);
                acc + if neighbours.len() == 2 {
                    neighbours.iter().product::<i32>()
                } else {
                    0
                }
            })
    })
}

fn read_input<R: BufRead>(reader: R) -> Vec<String> {
    reader.lines().map_while(Result::ok).collect()
}

fn get_neighbours(x: usize, y: usize, input: &[String]) -> Vec<char> {
    let mut neighbours = Vec::new();
    let mut push = |y: usize, x: usize| {
        neighbours.push(input[y].chars().nth(x).unwrap());
    };

    if x > 0 {
        push(y, x - 1);
    }
    if x < input[y].len() - 1 {
        push(y, x + 1);
    }
    if y > 0 {
        push(y - 1, x);
    }
    if y < input.len() - 1 {
        push(y + 1, x);
    }
    if x > 0 && y > 0 {
        push(y - 1, x - 1);
    }
    if x < input[y].len() - 1 && y > 0 {
        push(y - 1, x + 1);
    }
    if x > 0 && y < input.len() - 1 {
        push(y + 1, x - 1);
    }
    if x < input[y].len() - 1 && y < input.len() - 1 {
        push(y + 1, x + 1);
    }
    neighbours
}

fn get_neighbour_numbers(x: usize, y: usize, input: &[String]) -> Vec<i32> {
    let mut neighbours = Vec::new();

    let get_left = |y: usize, x: usize| {
        let first_digit_offset = input[y]
            .chars()
            .rev()
            .skip(input[y].len() - x)
            .position(|c| !c.is_ascii_digit())
            .unwrap_or(x);

        input[y]
            .chars()
            .skip(x - first_digit_offset)
            .take_while(|&c| c.is_ascii_digit())
            .collect::<String>()
    };

    let get_right = |y: usize, x: usize| {
        input[y]
            .chars()
            .skip(x + 1)
            .take_while(|&c| c.is_ascii_digit())
            .collect::<String>()
    };

    neighbours.push(get_left(y, x));
    neighbours.push(get_right(y, x));

    if y > 0 && input[y - 1].chars().nth(x).unwrap().is_ascii_digit() {
        neighbours.push(get_left(y - 1, x));
    } else if y > 0 {
        neighbours.push(get_left(y - 1, x));
        neighbours.push(get_right(y - 1, x));
    }
    if y < input.len() - 1 && input[y + 1].chars().nth(x).unwrap().is_ascii_digit() {
        neighbours.push(get_left(y + 1, x));
    } else if y < input.len() - 1 {
        neighbours.push(get_left(y + 1, x));
        neighbours.push(get_right(y + 1, x));
    }

    neighbours.iter().filter_map(|n| n.parse().ok()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::File, io::BufReader};

    #[test]
    fn test_solution() {
        let input = BufReader::new(File::open("inputs/day3/input.txt").unwrap());
        let input = read_input(input);
        assert_eq!(527369, part1(&input));
        assert_eq!(73074886, part2(&input));
    }

    #[test]
    fn test_example_1() {
        let input = BufReader::new(File::open("inputs/day3/example.txt").unwrap());
        let input = read_input(input);
        assert_eq!(4361, part1(&input));
    }

    #[test]
    fn test_example_2() {
        let input = BufReader::new(File::open("inputs/day3/example.txt").unwrap());
        let input = read_input(input);
        assert_eq!(467835, part2(&input));
    }
}
