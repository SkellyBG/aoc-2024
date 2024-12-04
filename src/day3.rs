use regex::Regex;

#[aoc(day3, part1)]
pub fn part1(input: &str) -> i32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    re.captures_iter(input)
        .map(|c| c[1].parse::<i32>().unwrap() * c[2].parse::<i32>().unwrap())
        .sum()
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> i32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();

    re.captures_iter(input)
        .fold((true, 0), |mut acc, c| {
            if c[0] == *"do()" {
                acc.0 = true;
            } else if c[0] == *"don't()" {
                acc.0 = false;
            } else if acc.0 {
                acc.1 += c[1].parse::<i32>().unwrap() * c[2].parse::<i32>().unwrap();
            }

            acc
        })
        .1
}
