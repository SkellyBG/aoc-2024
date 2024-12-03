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

    re.captures_iter(input)
        .map(|m| m.extract::<2>())
        .fold((true, 0), |mut acc, (instruction, [l, r])| {
            if instruction == "do()" {
                acc.0 = true;
            } else if instruction == "don't()" {
                acc.0 = false;
            } else if acc.0 {
                acc.1 += l.parse::<i32>().unwrap() * r.parse::<i32>().unwrap();
            }

            acc
        })
        .1
}
