use itertools::Itertools;
use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};

use crate::grid::Grid;

#[aoc_generator(day8)]
fn parse(input: &str) -> (Grid, Vec<Vec<(usize, usize)>>) {
    let grid = Grid::new(input);

    let mut nodes = Vec::new();

    let mut processed = HashSet::new();

    for row in grid.cells.iter() {
        for cell in row.iter() {
            if *cell != b'.' && !processed.contains(&cell) {
                nodes.push(grid.find_all(*cell));
                processed.insert(cell);
            }
        }
    }

    (grid, nodes)
}

fn distance(a: (usize, usize), b: (usize, usize)) -> usize {
    let (x1, y1) = a;
    let (x2, y2) = b;

    ((x1 as i32 - x2 as i32).abs() + (y1 as i32 - y2 as i32).abs()) as usize
}

fn is_collinear(a: (usize, usize), b: (usize, usize), c: (usize, usize)) -> bool {
    let (x1, y1) = a;
    let (x2, y2) = b;
    let (x3, y3) = c;

    (y3 as i32 - y1 as i32) * (x2 as i32 - x1 as i32)
        == (y2 as i32 - y1 as i32) * (x3 as i32 - x1 as i32)
}

#[aoc(day8, part1)]
fn part1(input: &(Grid, Vec<Vec<(usize, usize)>>)) -> i32 {
    let (grid, nodes) = input;

    let mut total = 0;

    let mut processed = HashSet::new();

    for (x, y, _) in grid.iter() {
        if nodes.iter().any(|node| {
            node.iter().permutations(2).any(|vec| {
                is_collinear((x, y), *vec[0], *vec[1])
                    && distance((x, y), *vec[0]) == 2 * distance((x, y), *vec[1])
            })
        }) {
            total += 1;
            processed.insert((x, y));
        }
    }

    total
}

#[aoc(day8, part2)]
fn part2(input: &(Grid, Vec<Vec<(usize, usize)>>)) -> i32 {
    let (grid, nodes) = input;

    let mut total = 0;

    let mut processed = HashSet::new();

    for (x, y, _) in grid.iter() {
        if nodes.iter().any(|node| {
            node.iter()
                .permutations(2)
                .any(|vec| is_collinear((x, y), *vec[0], *vec[1]))
        }) {
            total += 1;
            processed.insert((x, y));
        }
    }

    total
}

#[cfg(test)]
const EXAMPLE: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 14);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 34);
    }
}
