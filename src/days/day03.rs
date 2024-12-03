use regex::Regex;

#[aoc(day3, part1)]
pub fn part1(input: &str) -> i32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut total = 0;
    for (_, [l, r]) in re.captures_iter(input).map(|m| m.extract()) {
        total += l.parse::<i32>().unwrap() * r.parse::<i32>().unwrap();
    }

    total
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> i32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)()()|don't\(\)()()").unwrap();

    let mut total = 0;

    let mut is_enabled = true;

    for (instruction, [l, r]) in re.captures_iter(input).map(|m| m.extract()) {
        if instruction == "do()" {
            is_enabled = true;
        } else if instruction == "don't()" {
            is_enabled = false;
        }

        if is_enabled && instruction != "do()" && instruction != "don't()" {
            total += l.parse::<i32>().unwrap() * r.parse::<i32>().unwrap();
        }
    }

    total
}
