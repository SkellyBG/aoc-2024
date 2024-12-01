use itertools::Itertools;

pub fn part1(input: &str) -> i32 {
    let mut total = 0;
    for line in input.split('\n') {
        let nums = line
            .split_ascii_whitespace()
            .map(|e| e.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        if nums.len() <= 1 {
            total += 1;
            continue;
        }

        let ordering = nums[0] < nums[1];

        let mut is_safe = true;

        for (a, b) in nums.iter().tuple_windows() {
            if (a < b) != ordering {
                is_safe = false;
            }

            if a.abs_diff(*b) == 0 || a.abs_diff(*b) > 3 {
                is_safe = false;
            }
        }
        total += if is_safe { 1 } else { 0 };
    }

    total
}

pub fn part2(input: &str) -> i32 {
    let mut total = 0;
    for line in input.split('\n') {
        let nums = line
            .split_ascii_whitespace()
            .map(|e| e.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        if nums.len() <= 1 {
            total += 1;
            continue;
        }

        let ordering = nums[0] < nums[1];

        let mut is_safe = true;
        let mut has_dampener = true;

        for (a, b) in nums.iter().tuple_windows() {
            if (a < b) != ordering {
                is_safe = has_dampener;
                has_dampener = false;
                continue;
            }

            if a.abs_diff(*b) == 0 || a.abs_diff(*b) > 3 {
                is_safe = has_dampener;
                has_dampener = false;
                continue;
            }
        }
        total += if is_safe { 1 } else { 0 };
    }

    total
}
