use std::{collections::VecDeque, vec};

use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day10)]
fn parse(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| line.as_bytes().into_iter().map(|c| c - b'0').collect())
        .collect()
}

const DIRS: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

fn bfs(x: usize, y: usize, input: &Vec<Vec<u8>>) -> i32 {
    let mut q = VecDeque::new();

    let height = input.len();
    let width = input[0].len();

    let mut visited = vec![vec![false; width]; height];
    q.push_back((x, y));
    visited[x][y] = true;

    let mut total = 0;

    // should iterate 9 times
    while !q.is_empty() {
        let size = q.len();

        for _ in 0..size {
            let (x, y) = q.pop_front().unwrap();

            if input[x][y] == 9 {
                total += 1;
                continue;
            }

            for (dx, dy) in DIRS.iter() {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;

                if nx < 0 || ny < 0 || nx >= height as i32 || ny >= width as i32 {
                    continue;
                }

                let nx = nx as usize;
                let ny = ny as usize;

                if visited[nx][ny] {
                    continue;
                }

                if input[nx][ny] == input[x][y] + 1 {
                    q.push_back((nx, ny));
                    visited[nx][ny] = true;
                }
            }
        }
    }

    total
}

fn dfs(x: usize, y: usize, input: &Vec<Vec<u8>>) -> i32 {
    if input[x][y] == 9 {
        return 1;
    }

    let height = input.len();
    let width = input[0].len();

    let mut total = 0;

    for (dx, dy) in DIRS.iter() {
        let nx = x as i32 + dx;
        let ny = y as i32 + dy;

        if nx < 0 || ny < 0 || nx >= height as i32 || ny >= width as i32 {
            continue;
        }

        let nx = nx as usize;
        let ny = ny as usize;

        if input[nx][ny] == input[x][y] + 1 {
            total += dfs(nx, ny, input);
        }
    }

    total
}

#[aoc(day10, part1)]
fn part1(input: &Vec<Vec<u8>>) -> i32 {
    let height = input.len();
    let width = input[0].len();

    let mut total = 0;

    for x in 0..height {
        for y in 0..width {
            if input[x][y] != 0 {
                continue;
            }

            total += bfs(x, y, input);
        }
    }

    total
}

#[aoc(day10, part2)]
fn part2(input: &Vec<Vec<u8>>) -> i32 {
    let height = input.len();
    let width = input[0].len();

    let mut total = 0;

    for x in 0..height {
        for y in 0..width {
            if input[x][y] != 0 {
                continue;
            }

            total += dfs(x, y, input);
        }
    }

    total
}

#[cfg(test)]
const EXAMPLE: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 36);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 81);
    }
}
