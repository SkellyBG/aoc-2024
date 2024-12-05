#[aoc(day5, part1)]
pub fn part1(input: &str) -> i32 {
    let mut edges = Vec::new();

    let mut instructions = Vec::new();

    for line in input.split('\n') {
        if line.contains('|') {
            let (l, r) = line.split_once('|').unwrap();
            edges.push((l.parse::<i32>().unwrap(), r.parse::<i32>().unwrap()))
        } else if line.is_empty() {
            continue;
        } else {
            instructions.push(
                line.split(',')
                    .map(|e| e.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>(),
            );
        }
    }

    let mut total = 0;

    for instruction in instructions.iter() {
        let mut flag = true;

        for (idx, l) in instruction.iter().enumerate() {
            for r in instruction.iter().skip(idx) {
                if edges.contains(&(*r, *l)) {
                    flag = false;
                }
            }
        }
        if flag {
            total += instruction[instruction.len() / 2];
        }
    }

    total
}

#[aoc(day5, part2)]
pub fn part2(input: &str) -> i32 {
    let mut edges = Vec::new();

    let mut instructions = Vec::new();

    for line in input.split('\n') {
        if line.contains('|') {
            let (l, r) = line.split_once('|').unwrap();
            edges.push((l.parse::<i32>().unwrap(), r.parse::<i32>().unwrap()))
        } else if line.is_empty() {
            continue;
        } else {
            instructions.push(
                line.split(',')
                    .map(|e| e.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>(),
            );
        }
    }

    let mut total = 0;

    for instruction in instructions.iter() {
        let mut flag = true;

        for (idx, l) in instruction.iter().enumerate() {
            for r in instruction.iter().skip(idx) {
                if edges.contains(&(*r, *l)) {
                    flag = false;
                }
            }
        }
        if !flag {
            let mut new_instruction = instruction.clone();

            loop {
                // println!("{:?}", new_instruction);
                if update(&mut new_instruction, &edges) {
                    break;
                }
            }

            total += new_instruction[new_instruction.len() / 2];
        }
    }

    total
}

fn update(v: &mut Vec<i32>, edges: &Vec<(i32, i32)>) -> bool {
    for (idx_l, l) in v.iter().enumerate() {
        for (idx_r, r) in v.iter().enumerate().skip(idx_l) {
            if edges.contains(&(*r, *l)) {
                v.swap(idx_l, idx_r);

                return false;
            }
        }
    }

    true
}
