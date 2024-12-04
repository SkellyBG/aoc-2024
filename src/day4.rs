#[aoc(day4, part1)]
pub fn part1(input: &str) -> i32 {
    let lines = input.split("\n").collect::<Vec<&str>>();

    let height = lines.len() as i32;
    let width = lines[0].len() as i32;

    let mut total = 0;

    let dirs = [
        (0, 1),
        (1, 0),
        (0, -1),
        (-1, 0),
        (1, 1),
        (-1, -1),
        (-1, 1),
        (1, -1),
    ];

    let word = "XMAS";

    for i in 0..height {
        for j in 0..width {
            for dir in dirs.iter() {
                let flag = (0..4).all(|offset| {
                    let x = i + dir.0 * offset;
                    let y = j + dir.1 * offset;

                    if x < 0 || x >= height || y < 0 || y >= width {
                        return false;
                    }

                    if lines[x as usize].chars().nth(y as usize).unwrap()
                        != word.chars().nth(offset as usize).unwrap()
                    {
                        return false;
                    }
                    true
                });

                total += if flag { 1 } else { 0 };
            }
        }
    }

    total
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> i32 {
    let lines = input.split("\n").collect::<Vec<&str>>();

    let height = lines.len() as i32;
    let width = lines[0].len() as i32;

    let mut total = 0;

    let dirs = [(1, 1), (1, -1)];
    for i in 0..height {
        for j in 0..width {
            if lines[i as usize].chars().nth(j as usize).unwrap() != 'A' {
                continue;
            }

            // check diagonals

            let flag = dirs.iter().all(|dir| {
                let x_1 = i + dir.0;
                let y_1 = j + dir.1;

                let x_2 = i - dir.0;
                let y_2 = j - dir.1;

                if x_1 < 0 || x_1 >= height || y_1 < 0 || y_1 >= width {
                    return false;
                }

                if x_2 < 0 || x_2 >= height || y_2 < 0 || y_2 >= width {
                    return false;
                }

                if lines[x_1 as usize].chars().nth(y_1 as usize).unwrap() == 'M'
                    && lines[x_2 as usize].chars().nth(y_2 as usize).unwrap() == 'S'
                    || lines[x_1 as usize].chars().nth(y_1 as usize).unwrap() == 'S'
                        && lines[x_2 as usize].chars().nth(y_2 as usize).unwrap() == 'M'
                {
                    return true;
                }

                false
            });

            total += if flag { 1 } else { 0 };
        }
    }

    total
}
