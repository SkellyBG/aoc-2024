use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day7)]
fn parse(input: &str) -> Vec<(i64, Vec<i64>)> {
    input
        .split('\n')
        .map(|line| {
            let (l, r) = line.split_once(": ").unwrap();
            (
                l.parse().unwrap(),
                r.split_ascii_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect(),
            )
        })
        .collect()
}

fn is_equation_valid(total: i64, nums: &[i64], running: i64) -> bool {
    if nums.is_empty() {
        return total == running;
    }

    is_equation_valid(total, &nums[1..], running + nums[0])
        || is_equation_valid(total, &nums[1..], running * nums[0])
}

fn is_equation_valid_2(total: i64, nums: &[i64], running: i128) -> bool {
    if running > i128::from(total) {
        return false;
    }

    if nums.is_empty() {
        return i128::from(total) == running;
    }

    is_equation_valid_2(total, &nums[1..], running + i128::from(nums[0]))
        || is_equation_valid_2(total, &nums[1..], running * i128::from(nums[0]))
        || is_equation_valid_2(
            total,
            &nums[1..],
            running * i128::from(10i64.pow(nums[0].ilog10() + 1u32)) + i128::from(nums[0]),
        )
}

#[aoc(day7, part1)]
fn part1(input: &[(i64, Vec<i64>)]) -> i64 {
    input
        .iter()
        .map(|line| {
            let (total, nums) = line;
            let var_name = if is_equation_valid(*total, &nums[1..], nums[0]) {
                *total
            } else {
                0
            };
            var_name
        })
        .sum()
}

#[aoc(day7, part2)]
fn part2(input: &[(i64, Vec<i64>)]) -> i64 {
    input
        .iter()
        .map(|line| {
            let (total, nums) = line;
            if is_equation_valid_2(*total, &nums[1..], i128::from(nums[0])) {
                *total
            } else {
                0
            }
        })
        .sum()
}

#[cfg(test)]
const EXAMPLE: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 3749);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 11387);
    }
}
