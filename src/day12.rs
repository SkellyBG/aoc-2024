use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day12)]
fn parse(input: &str) -> Vec<Vec<u8>> {
    input.split('\n').map(|f| f.bytes().collect()).collect()
}

const DIRS: [(i32, i32); 4] = [(0, -1), (0, 1), (1, 0), (-1, 0)];

fn bfs(x: usize, y: usize, visited: &mut Vec<Vec<bool>>, input: &Vec<Vec<u8>>) -> (i64, i64) {
    if visited[x][y] {
        return (0, 0);
    }

    visited[x][y] = true;

    let height = visited.len();
    let width = visited[0].len();

    let mut area = 1;
    let mut perimeter = 0;

    for (dx, dy) in DIRS {
        let nx = i32::try_from(x).unwrap() + dx;
        let ny = i32::try_from(y).unwrap() + dy;

        if nx < 0
            || nx >= height.try_into().unwrap()
            || ny < 0
            || ny >= width.try_into().unwrap()
            || input[nx as usize][ny as usize] != input[x][y]
        {
            perimeter += 1;
            continue;
        }

        let (a, p) = bfs(nx as usize, ny as usize, visited, input);

        area += a;
        perimeter += p;
    }

    (area, perimeter)
}

#[aoc(day12, part1)]
fn part1(input: &Vec<Vec<u8>>) -> i64 {
    let height = input.len();
    let width = input[0].len();

    let mut visited = vec![vec![false; width]; height];

    let mut total = 0;

    for x in 0..height {
        for y in 0..width {
            let (a, p) = bfs(x, y, &mut visited, &input);
            total += a * p;
        }
    }

    total
}

#[aoc(day12, part2)]
fn part2(input: &Vec<Vec<u8>>) -> i64 {
    todo!()
}

#[cfg(test)]
const EXAMPLE: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 1930);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 1206);
    }
}
