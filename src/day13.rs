use aoc_runner_derive::{aoc, aoc_generator};
use std::cmp::Reverse;

#[aoc_generator(day13)]
fn parse(input: &str) -> Vec<((i32, i32), (i32, i32), (i32, i32))> {
    let re = regex::Regex::new(r"X\+(\d+), Y\+(\d+)").unwrap();
    let re2 = regex::Regex::new(r"X=(\d+), Y=(\d+)").unwrap();

    input
        .split("\n\n")
        .map(|f| {
            let mut iter = f.split('\n');
            let first = iter.next().unwrap();
            let second = iter.next().unwrap();
            let third = iter.next().unwrap();

            let (x1, y1) = re
                .captures_iter(first)
                .map(|f| (f[1].parse().unwrap(), f[2].parse().unwrap()))
                .nth(0)
                .unwrap();

            let (x2, y2) = re
                .captures_iter(second)
                .map(|f| (f[1].parse().unwrap(), f[2].parse().unwrap()))
                .nth(0)
                .unwrap();

            let (x3, y3) = re2
                .captures_iter(third)
                .map(|f| (f[1].parse().unwrap(), f[2].parse().unwrap()))
                .nth(0)
                .unwrap();

            ((x1, y1), (x2, y2), (x3, y3))
        })
        .collect()
}

#[aoc(day13, part1)]
fn part1(input: &[((i32, i32), (i32, i32), (i32, i32))]) -> i32 {
    let mut total = 0;

    for (a, b, prize) in input {
        let mut pq = std::collections::BinaryHeap::new();

        let mut cache = std::collections::HashSet::new();

        pq.push(Reverse((0, 0, 0)));

        loop {
            let item = pq.pop().unwrap().0;

            if item.0 > 400 {
                break;
            }

            if cache.contains(&(item.1, item.2)) {
                continue;
            }
            cache.insert((item.1, item.2));

            if item.1 == prize.0 && item.2 == prize.1 {
                total += item.0;
                break;
            }

            pq.push(Reverse((item.0 + 3, item.1 + a.0, item.2 + a.1)));
            pq.push(Reverse((item.0 + 1, item.1 + b.0, item.2 + b.1)));
        }
    }

    total
}

#[aoc(day13, part2)]
fn part2(input: &[((i32, i32), (i32, i32), (i32, i32))]) -> i32 {
    let oop = 10000000000000_i64;

    let mut total = 0;

    for (a, b, prize) in input {
        let mut pq = std::collections::BinaryHeap::new();

        let mut cache = std::collections::HashSet::new();

        pq.push(Reverse((0, 0, 0)));

        loop {
            let item = pq.pop().unwrap().0;

            if item.0 > 400 {
                break;
            }

            if cache.contains(&(item.1, item.2)) {
                continue;
            }
            cache.insert((item.1, item.2));

            if item.1 == prize.0 && item.2 == prize.1 {
                total += item.0;
                break;
            }

            pq.push(Reverse((item.0 + 3, item.1 + a.0, item.2 + a.1)));
            pq.push(Reverse((item.0 + 1, item.1 + b.0, item.2 + b.1)));
        }
    }

    total
}

const EXAMPLE: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 480);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), 0);
    }
}
