use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day11)]
fn parse(input: &str) -> Vec<i128> {
    input
        .split_ascii_whitespace()
        .map(|l| l.parse().unwrap())
        .collect()
}

enum Update {
    Singular(i128),
    Double(i128, i128),
}

fn digit_count(v: i128) -> u32 {
    if v == 0 {
        return 1;
    }
    return v.ilog10() + 1;
}

fn update(v: i128) -> Update {
    let digit_count = digit_count(v);

    if v == 0 {
        Update::Singular(1)
    } else if digit_count % 2 == 0 {
        let l = v / 10i128.pow(digit_count / 2);

        Update::Double(l, v - l * 10i128.pow(digit_count / 2))
    } else {
        Update::Singular(v * 2024)
    }
}

#[aoc(day11, part1)]
fn part1(input: &Vec<i128>) -> usize {
    let mut new_input = input.clone();

    for _ in 0..25 {
        new_input = new_input
            .iter()
            .map(|v| match update(*v) {
                Update::Singular(v) => vec![v],
                Update::Double(l, r) => vec![l, r],
            })
            .flatten()
            .collect();
        // eprint!("{:?}\n", new_input);
    }

    new_input.len()
}

#[aoc(day11, part2)]
fn part2(input: &Vec<i128>) -> i128 {
    let mut new_input: Vec<(i128, i128)> = input.clone().into_iter().map(|f| (f, 1)).collect();

    for _ in 0..75 {
        new_input = new_input
            .iter()
            .map(|(v, count)| match update(*v) {
                Update::Singular(v) => vec![(v, *count)],
                Update::Double(l, r) => vec![(l, *count), (r, *count)],
            })
            .flatten()
            .collect();

        let mut dedupe = HashMap::new();
        for (v, count) in new_input.iter() {
            let entry = dedupe.entry(v).or_insert(0);
            *entry += count;
        }

        new_input = dedupe.into_iter().map(|f| (*f.0, f.1)).collect();
    }

    new_input.into_iter().map(|f| f.1).sum()
}

#[cfg(test)]
const EXAMPLE: &str = "125 17";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pqart1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 55312);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 55312);
    }
}
