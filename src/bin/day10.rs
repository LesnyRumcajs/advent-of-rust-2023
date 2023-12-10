use std::{collections::HashSet, io::BufRead};

fn main() {
    let input = read_input(std::io::stdin().lock());
    let solution = solve(&input);
    println!("Day 10, part 1: {}", solution.0);

    // part 2 is solved organoleptically with Vim
    //for row in solution.1 {
    //    println!("{}", row.iter().collect::<String>());
    //}
}

fn solve(input: &[Vec<char>]) -> (i32, Vec<Vec<char>>) {
    let start = input
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter().enumerate().find_map(|(x, c)| {
                if *c == 'S' {
                    Some(Point { x, y })
                } else {
                    None
                }
            })
        })
        .unwrap();

    let input = interpolate(&start, input);

    let mut visited: HashSet<Point> = HashSet::new();
    let mut current = start;
    while current != start || visited.is_empty() {
        let (left_neighbour, right_neighbour, top_neighbour, bottom_neighbour) =
            get_neighbours_not_visited(&current, &input, &visited);

        let current_type = input[current.y][current.x];
        let next = match current_type {
            '|' => {
                if matches!(top_neighbour, Some(x) if ['F', '7', '|'].contains(&x)) {
                    Point {
                        x: current.x,
                        y: current.y - 1,
                    }
                } else if matches!(bottom_neighbour, Some(x) if ['L', 'J', '|'].contains(&x)) {
                    Point {
                        x: current.x,
                        y: current.y + 1,
                    }
                } else {
                    break;
                }
            }
            '-' => {
                if matches!(left_neighbour, Some(x) if ['F', 'L', '-'].contains(&x)) {
                    Point {
                        x: current.x - 1,
                        y: current.y,
                    }
                } else if matches!(right_neighbour, Some(x) if ['J', '-', '7'].contains(&x)) {
                    Point {
                        x: current.x + 1,
                        y: current.y,
                    }
                } else {
                    break;
                }
            }
            'F' => {
                if matches!(right_neighbour, Some(x) if ['-', '7', 'J'].contains(&x)) {
                    Point {
                        x: current.x + 1,
                        y: current.y,
                    }
                } else if matches!(bottom_neighbour, Some(x) if ['L', 'J', '|'].contains(&x)) {
                    Point {
                        x: current.x,
                        y: current.y + 1,
                    }
                } else {
                    break;
                }
            }
            'J' => {
                if matches!(top_neighbour, Some(x) if ['|', 'F', '7'].contains(&x)) {
                    Point {
                        x: current.x,
                        y: current.y - 1,
                    }
                } else if matches!(left_neighbour, Some(x) if ['L', '-', 'F'].contains(&x)) {
                    Point {
                        x: current.x - 1,
                        y: current.y,
                    }
                } else {
                    break;
                }
            }
            '7' => {
                if matches!(left_neighbour, Some(x) if ['L', '-', 'F'].contains(&x)) {
                    Point {
                        x: current.x - 1,
                        y: current.y,
                    }
                } else if matches!(bottom_neighbour, Some(x) if ['L', 'J', '|'].contains(&x)) {
                    Point {
                        x: current.x,
                        y: current.y + 1,
                    }
                } else {
                    break;
                }
            }
            'L' => {
                if matches!(right_neighbour, Some(x) if ['-', '7', 'J'].contains(&x)) {
                    Point {
                        x: current.x + 1,
                        y: current.y,
                    }
                } else if matches!(top_neighbour, Some(x) if ['|', 'F', '7'].contains(&x)) {
                    Point {
                        x: current.x,
                        y: current.y - 1,
                    }
                } else {
                    break;
                }
            }
            _ => panic!("Invalid input"),
        };

        visited.insert(current);
        current = next;
    }

    let pipes = input
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, c)| {
                    if visited.contains(&Point { x, y }) {
                        *c
                    } else {
                        ' '
                    }
                })
                .collect()
        })
        .collect();
    ((visited.len() as i32 + 1) / 2, pipes)
}

// left, right, top, bottom
fn get_neighbours_not_visited(
    point: &Point,
    input: &[Vec<char>],
    visited: &HashSet<Point>,
) -> (Option<char>, Option<char>, Option<char>, Option<char>) {
    let get_not_visited = |x, y| {
        if visited.contains(&Point { x, y }) {
            None
        } else {
            Some(input[y][x])
        }
    };

    let left = if point.x > 0 {
        get_not_visited(point.x - 1, point.y)
    } else {
        None
    };

    let right = if point.x < input[point.y].len() - 1 {
        get_not_visited(point.x + 1, point.y)
    } else {
        None
    };

    let top = if point.y > 0 {
        get_not_visited(point.x, point.y - 1)
    } else {
        None
    };

    let bottom = if point.y < input.len() - 1 {
        get_not_visited(point.x, point.y + 1)
    } else {
        None
    };
    (left, right, top, bottom)
}

fn interpolate(point: &Point, input: &[Vec<char>]) -> Vec<Vec<char>> {
    let (left_neighbour, right_neighbour, top_neighbour, bottom_neighbour) =
        get_neighbours_not_visited(point, input, &HashSet::new());

    // not all cases covered
    let p = match (
        left_neighbour,
        right_neighbour,
        top_neighbour,
        bottom_neighbour,
    ) {
        (Some('-'), Some('-'), _, _) => '-',
        (_, _, Some('|'), Some('|')) => '|',
        (_, Some('-'), _, Some('|')) => 'F',
        (Some('-'), _, _, Some('|')) => 'J',
        (_, Some('7'), Some('F'), _) => 'L',
        (_, Some('J'), _, Some('|')) => 'F',
        _ => panic!("Invalid input"),
    };

    let mut result = input.to_vec();
    result[point.y][point.x] = p;
    result
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

fn read_input<R: BufRead>(reader: R) -> Vec<Vec<char>> {
    reader
        .lines()
        .map(|l| l.unwrap().chars().collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::File, io::BufReader};

    #[test]
    fn test_solution() {
        let input = BufReader::new(File::open("inputs/day10/input.txt").unwrap());

        let input = read_input(input);
        let (part1, _) = solve(&input);
        assert_eq!(6875, part1);
        // part 2 is solved organoleptically with Vim
        // :'<,'>s/\%V\ //gn
    }

    #[test]
    fn test_example() {
        let input = BufReader::new(File::open("inputs/day10/example2.txt").unwrap());

        let input = read_input(input);
        let (part1, _) = solve(&input);
        assert_eq!(8, part1);
    }
}
