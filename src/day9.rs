use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone)]
enum FileBlock {
    File { size: u8, id: usize },
    FreeSpace(u8),
}

#[aoc_generator(day9)]
fn parse(input: &str) -> Vec<FileBlock> {
    let bytes = input.as_bytes();
    bytes
        .chunks(2)
        .enumerate()
        .map(|(id, chunk)| {
            let mut a = vec![FileBlock::File {
                size: chunk[0] - b'0',
                id,
            }];
            if let Some(size) = chunk.get(1) {
                a.push(FileBlock::FreeSpace(*size));
            }
            a
        })
        .flatten()
        .collect()
}

#[aoc(day9, part1)]
fn part1(input: &Vec<FileBlock>) -> i32 {
    let mut input = input.clone();

    loop {}
}

#[aoc(day9, part2)]
fn part2(input: &Vec<FileBlock>) -> String {
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
