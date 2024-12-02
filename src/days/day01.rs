use std::iter::zip;

use rustc_hash::FxHashMap;

const LINE_BYTES_LENGTH: usize = 14;

#[aoc(day1, part1)]
pub fn part1(input: &str) -> i32 {
    let mut list1: Vec<i32> = Vec::with_capacity(1000);
    let mut list2: Vec<i32> = Vec::with_capacity(1000);

    let bytes = input.as_bytes();

    for i in 0..1000 {
        let l = (bytes[i * LINE_BYTES_LENGTH] - b'0') as i32 * 10000
            + (bytes[i * LINE_BYTES_LENGTH + 1] - b'0') as i32 * 1000
            + (bytes[i * LINE_BYTES_LENGTH + 2] - b'0') as i32 * 100
            + (bytes[i * LINE_BYTES_LENGTH + 3] - b'0') as i32 * 10
            + (bytes[i * LINE_BYTES_LENGTH + 4] - b'0') as i32;

        let r = (bytes[i * LINE_BYTES_LENGTH + 8] - b'0') as i32 * 10000
            + (bytes[i * LINE_BYTES_LENGTH + 9] - b'0') as i32 * 1000
            + (bytes[i * LINE_BYTES_LENGTH + 10] - b'0') as i32 * 100
            + (bytes[i * LINE_BYTES_LENGTH + 11] - b'0') as i32 * 10
            + (bytes[i * LINE_BYTES_LENGTH + 12] - b'0') as i32;

        list1.push(l);
        list2.push(r);
    }

    list1.sort_unstable();
    list2.sort_unstable();

    zip(list1, list2).fold(0, |acc, (l, r)| acc + (l - r).abs())
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> i32 {
    let mut list = Vec::with_capacity(1000);
    let mut map = FxHashMap::default();
    map.reserve(1000);

    let bytes = input.as_bytes();

    for i in 0..1000 {
        let l = (bytes[i * LINE_BYTES_LENGTH] - b'0') as i32 * 10000
            + (bytes[i * LINE_BYTES_LENGTH + 1] - b'0') as i32 * 1000
            + (bytes[i * LINE_BYTES_LENGTH + 2] - b'0') as i32 * 100
            + (bytes[i * LINE_BYTES_LENGTH + 3] - b'0') as i32 * 10
            + (bytes[i * LINE_BYTES_LENGTH + 4] - b'0') as i32;

        let r = (bytes[i * LINE_BYTES_LENGTH + 8] - b'0') as i32 * 10000
            + (bytes[i * LINE_BYTES_LENGTH + 9] - b'0') as i32 * 1000
            + (bytes[i * LINE_BYTES_LENGTH + 10] - b'0') as i32 * 100
            + (bytes[i * LINE_BYTES_LENGTH + 11] - b'0') as i32 * 10
            + (bytes[i * LINE_BYTES_LENGTH + 12] - b'0') as i32;

        list.push(l);
        map.entry(r).and_modify(|e| *e += 1).or_insert(1);
    }

    list.iter()
        .fold(0, |acc, l| acc + l * map.get(l).unwrap_or(&0))
}
