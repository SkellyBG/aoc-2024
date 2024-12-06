use core::panic;
use std::collections::HashSet;

#[derive(Clone)]
struct Guard {
    direction: (i32, i32),
    position: (i32, i32),
}

#[aoc(day6, part1)]
fn part1(input: &str) -> i32 {
    let lines = input
        .split("\n")
        .map(|v| v.as_bytes())
        .collect::<Vec<&[u8]>>();

    let height = lines.len() as i32;
    let width = lines[0].len() as i32;

    let mut guard = Guard {
        direction: (0, 1),
        position: (0, 0),
    };

    for (idx_x, x) in lines.iter().enumerate() {
        for (idx_y, y) in x.iter().enumerate() {
            if *y == b'^' {
                guard = Guard {
                    direction: (-1, 0),
                    position: (idx_x as i32, idx_y as i32),
                };
            }
        }
    }
    let mut total = 1;

    let mut visited = HashSet::new();

    visited.insert(guard.position);

    loop {
        let x = guard.position.0 + guard.direction.0;
        let y = guard.position.1 + guard.direction.1;

        if x < 0 || x >= height || y < 0 || y >= width {
            break;
        }

        if lines[x as usize][y as usize] == b'#' {
            match guard.direction {
                (0, 1) => {
                    guard.direction = (1, 0);
                }
                (0, -1) => {
                    guard.direction = (-1, 0);
                }
                (1, 0) => {
                    guard.direction = (0, -1);
                }
                (-1, 0) => {
                    guard.direction = (0, 1);
                }
                _ => panic!(),
            };
        }

        let x = guard.position.0 + guard.direction.0;
        let y = guard.position.1 + guard.direction.1;

        if x < 0 || x >= height || y < 0 || y >= width {
            break;
        }

        if !visited.contains(&(x, y)) {
            total += 1;
            visited.insert((x, y));
        }

        guard.position = (x, y);
    }

    total
}

#[aoc(day6, part2)]
fn part2(input: &str) -> i32 {
    let lines = input
        .split("\n")
        .map(|v| String::from(v).into_bytes())
        .collect::<Vec<Vec<u8>>>();

    let mut guard = Guard {
        direction: (0, 1),
        position: (0, 0),
    };

    for (idx_x, x) in lines.iter().enumerate() {
        for (idx_y, y) in x.iter().enumerate() {
            if *y == b'^' {
                guard = Guard {
                    direction: (-1, 0),
                    position: (idx_x as i32, idx_y as i32),
                };
            }
        }
    }

    let mut total = 0;

    for (idx_x, x) in lines.iter().enumerate() {
        for (idx_y, y) in x.iter().enumerate() {
            let mut obstructed = lines.clone();
            if *y == b'#' || *y == b'^' {
                continue;
            }

            obstructed[idx_x][idx_y] = b'#';

            if has_loop(guard.clone(), obstructed) {
                total += 1;
            }
        }
    }

    total
}

fn has_loop(mut guard: Guard, obstructed: Vec<Vec<u8>>) -> bool {
    let mut visited = HashSet::new();

    let height = obstructed.len() as i32;
    let width = obstructed[0].len() as i32;

    loop {
        if visited.contains(&(guard.position, guard.direction)) {
            return true;
        }

        visited.insert((guard.position, guard.direction));

        let mut x = guard.position.0 + guard.direction.0;
        let mut y = guard.position.1 + guard.direction.1;

        if x < 0 || x >= height || y < 0 || y >= width {
            break;
        }

        while obstructed[x as usize][y as usize] == b'#' {
            match guard.direction {
                (0, 1) => {
                    guard.direction = (1, 0);
                }
                (0, -1) => {
                    guard.direction = (-1, 0);
                }
                (1, 0) => {
                    guard.direction = (0, -1);
                }
                (-1, 0) => {
                    guard.direction = (0, 1);
                }
                _ => panic!(),
            };
            x = guard.position.0 + guard.direction.0;
            y = guard.position.1 + guard.direction.1;
        }

        if x < 0 || x >= height || y < 0 || y >= width {
            break;
        }

        guard.position = (x, y);
    }

    false
}

const example: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(example), 41);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(example), 6);
    }
}
