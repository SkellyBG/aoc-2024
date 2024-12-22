use std::collections::{HashSet, VecDeque};

use aoc_runner_derive::{aoc, aoc_generator};

use crate::grid::Grid;
#[aoc_generator(day18)]
fn parse(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .map(|l| {
            let mut it = l.split(',').map(|v| v.parse().unwrap());
            (it.next().unwrap(), it.next().unwrap())
        })
        .collect()
}

fn bfs(start: (usize, usize), end: (usize, usize), grid: &Grid) -> Option<i32> {
    let mut q = VecDeque::new();

    let mut visited = HashSet::new();

    q.push_back(start);
    visited.insert(start);

    let mut round = 1;
    while !q.is_empty() {
        let size = q.len();

        for _ in 0..size {
            let cur = q.pop_front().unwrap();

            if cur == end {
                return Some(round);
            }

            for (dx, dy) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let new_x = cur.0 as i32 + dx;
                let new_y = cur.1 as i32 + dy;

                if let Some(cell) = grid.index(new_x, new_y) {
                    if cell == b'#' {
                        continue;
                    }

                    let new_pos = (new_x as usize, new_y as usize);

                    if visited.contains(&new_pos) {
                        continue;
                    }

                    q.push_back(new_pos);
                    visited.insert(new_pos);
                }
            }
        }

        round += 1;
    }

    None
}

#[aoc(day18, part1)]
fn part1(input: &Vec<(usize, usize)>) -> i32 {
    let grid_size = 71;

    let mut grid = Grid::new(
        &(0..grid_size)
            .map(|_| ".".repeat(grid_size))
            .collect::<Vec<String>>()
            .join("\n"),
    );

    input.iter().take(1024).for_each(|(x, y)| {
        grid.cells[*y][*x] = b'#';
    });

    bfs((0, 0), (grid_size - 1, grid_size - 1), &grid).unwrap() - 1
}

#[aoc(day18, part2)]
fn part2(input: &Vec<(usize, usize)>) -> String {
    let grid_size = 71;

    let mut grid = Grid::new(
        &(0..grid_size)
            .map(|_| ".".repeat(grid_size))
            .collect::<Vec<String>>()
            .join("\n"),
    );

    for i in input.iter() {
        grid.cells[i.1][i.0] = b'#';

        if let None = bfs((0, 0), (grid_size - 1, grid_size - 1), &grid) {
            return format!("{},{}", i.0, i.1);
        }
    }

    return "".to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 22);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }
}
