use std::iter::zip;
const LINE_BYTES_LENGTH: usize = 14;

#[aoc(day1, part1)]
pub fn part1(input: &str) -> u32 {
    let mut list1: Vec<u32> = Vec::with_capacity(1000);
    let mut list2: Vec<u32> = Vec::with_capacity(1000);

    let bytes = input.as_bytes();

    for i in 0..1000 {
        let l = (bytes[i * LINE_BYTES_LENGTH] - b'0') as u32 * 10000
            + (bytes[i * LINE_BYTES_LENGTH + 1] - b'0') as u32 * 1000
            + (bytes[i * LINE_BYTES_LENGTH + 2] - b'0') as u32 * 100
            + (bytes[i * LINE_BYTES_LENGTH + 3] - b'0') as u32 * 10
            + (bytes[i * LINE_BYTES_LENGTH + 4] - b'0') as u32;

        let r = (bytes[i * LINE_BYTES_LENGTH + 8] - b'0') as u32 * 10000
            + (bytes[i * LINE_BYTES_LENGTH + 9] - b'0') as u32 * 1000
            + (bytes[i * LINE_BYTES_LENGTH + 10] - b'0') as u32 * 100
            + (bytes[i * LINE_BYTES_LENGTH + 11] - b'0') as u32 * 10
            + (bytes[i * LINE_BYTES_LENGTH + 12] - b'0') as u32;

        list1.push(l);
        list2.push(r);
    }

    list1.sort_unstable();
    list2.sort_unstable();

    zip(list1, list2).fold(0, |acc, (l, r)| acc + l.abs_diff(r))
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> u32 {
    let mut list = Vec::with_capacity(1000);
    let mut map = [0u32; 100000];

    let bytes = input.as_bytes();

    for i in 0..1000 {
        let l = (bytes[i * LINE_BYTES_LENGTH] - b'0') as u32 * 10000
            + (bytes[i * LINE_BYTES_LENGTH + 1] - b'0') as u32 * 1000
            + (bytes[i * LINE_BYTES_LENGTH + 2] - b'0') as u32 * 100
            + (bytes[i * LINE_BYTES_LENGTH + 3] - b'0') as u32 * 10
            + (bytes[i * LINE_BYTES_LENGTH + 4] - b'0') as u32;

        let r = (bytes[i * LINE_BYTES_LENGTH + 8] - b'0') as u32 * 10000
            + (bytes[i * LINE_BYTES_LENGTH + 9] - b'0') as u32 * 1000
            + (bytes[i * LINE_BYTES_LENGTH + 10] - b'0') as u32 * 100
            + (bytes[i * LINE_BYTES_LENGTH + 11] - b'0') as u32 * 10
            + (bytes[i * LINE_BYTES_LENGTH + 12] - b'0') as u32;

        list.push(l);
        map[r as usize] += 1;
    }

    list.iter().fold(0, |acc, l| {
        acc + l * { unsafe { map.get_unchecked(*l as usize) } }
    })
}
