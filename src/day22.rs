use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day22)]
fn parse(input: &str) -> Vec<i64> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

const MOD: i64 = 16777216;

fn evolve(input: i64) -> i64 {
    let input = prune((input * 64) ^ input);
    let input = prune((input / 32) ^ input);
    prune((input * 2048) ^ input)
}

fn prune(input: i64) -> i64 {
    input % MOD
}

#[aoc(day22, part1)]
fn part1(input: &Vec<i64>) -> i64 {
    input
        .iter()
        .map(|i| {
            let mut input = *i;
            for _ in 0..2000 {
                input = evolve(input);
            }
            input
        })
        .sum()
}

#[aoc(day22, part2)]
fn part2(input: &Vec<i64>) -> i64 {
    // how many different changes can there be?
    // 19 ^ 4 = 130321
    let mut map = HashMap::new();

    input.iter().fold(&mut map, |acc, v| {
        // generate 2000 secret numbers from v
        // store the changes into a hashmap - tuple -> value
        // apply the changes to the outer map - tuple -> total value
        // find highest total value

        let mut inner_map = HashMap::new();

        let mut prev = *v;

        let mut diffs = Vec::new();

        for _ in 0..2000 {
            let cur = evolve(prev);

            diffs.push((cur % 10) - (prev % 10));

            if diffs.len() >= 4 {
                let diff = (
                    diffs[diffs.len() - 4],
                    diffs[diffs.len() - 3],
                    diffs[diffs.len() - 2],
                    diffs[diffs.len() - 1],
                );

                inner_map.entry(diff).or_insert(cur % 10);
            }

            prev = cur;
        }

        // apply the changes to the outer map
        for (k, v) in inner_map {
            *acc.entry(k).or_insert(0) += v;
        }

        acc
    });

    *map.iter().max_by_key(|(_, v)| **v).unwrap().1
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "1
10
100
2024";

    const EXAMPLE2: &str = "1
2
3
2024";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 37327623);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE2)), 23);
    }
}
