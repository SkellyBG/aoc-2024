use std::cell::Cell;

use regex::Regex;

#[aoc(day3, part1)]
pub fn part1(input: &str) -> i32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    re.captures_iter(input)
        .map(|m| m.extract())
        .map(|(_, [l, r])| l.parse::<i32>().unwrap() * r.parse::<i32>().unwrap())
        .sum()
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> i32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)()()|don't\(\)()()").unwrap();
    let is_enabled = Cell::new(true);

    re.captures_iter(input).map(|m| m.extract::<2>()).map(|(instruction, [l, r])| {
        if instruction == "do()" {
            is_enabled.replace(true);
        } else if instruction == "don't()" {
            is_enabled.replace(false);
        }
        (instruction, [l, r])
    }).map(|(instruction, [l, r])| {
        if is_enabled.get() && instruction != "do()" && instruction != "don't()" {
            l.parse::<i32>().unwrap() * r.parse::<i32>().unwrap()
        } else {
            0
        }
    }).sum::<i32>()
}
