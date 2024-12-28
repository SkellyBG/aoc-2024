use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone, Debug, PartialEq)]
enum Block {
    File { id: i32 },
    Free,
}

#[aoc_generator(day9)]
fn parse(input: &str) -> Vec<Block> {
    input
        .as_bytes()
        .into_iter()
        .enumerate()
        .flat_map(|(i, v)| {
            if i % 2 == 0 {
                vec![
                    Block::File {
                        id: i32::try_from(i).unwrap() / 2,
                    };
                    usize::from(*v - b'0')
                ]
            } else {
                vec![Block::Free; usize::from(*v - b'0')]
            }
        })
        .collect()
}

#[aoc(day9, part1)]
fn part1(input: &Vec<Block>) -> i64 {
    let mut input = input.clone();

    let mut left_idx = 0;
    let mut right_idx = input.len() - 1;

    loop {
        while let Some(Block::File { .. }) = input.get(left_idx) {
            left_idx += 1;
        }

        while let Some(Block::Free) = input.get(right_idx) {
            right_idx -= 1;
        }

        if right_idx <= left_idx {
            break;
        }

        input.swap(left_idx, right_idx);
    }

    input
        .iter()
        .enumerate()
        .map(|(idx, b)| match b {
            Block::File { id } => i64::try_from(idx).unwrap() * i64::from(*id),
            Block::Free => 0,
        })
        .sum()
}

#[aoc(day9, part2)]
fn part2(_input: &Vec<Block>) -> String {
    todo!()
}

#[cfg(test)]
const EXAMPLE: &str = "2333133121414131402";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 1928);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), "<RESULT>");
    }
}
