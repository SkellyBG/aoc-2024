use std::{collections::HashMap, iter::zip};

pub fn solve_pt1(input: &str) -> i32 {
    let mut list1 = Vec::with_capacity(1000);
    let mut list2 = Vec::with_capacity(1000);

    for line in input.split('\n') {
        let mut iter = line.split_ascii_whitespace();
        list1.push(iter.next().unwrap().parse::<i32>().unwrap());
        list2.push(iter.next().unwrap().parse::<i32>().unwrap());
    }

    list1.sort();
    list2.sort();

    zip(list1, list2).fold(0, |acc, (l, r)| acc + (l - r).abs())
}

pub fn solve_pt2(input: &str) -> i32 {
    let mut list = Vec::with_capacity(1000);
    let mut map = HashMap::new();

    for line in input.split('\n') {
        let mut iter = line.split_ascii_whitespace();
        list.push(iter.next().unwrap().parse::<i32>().unwrap());

        map.entry(iter.next().unwrap().parse::<i32>().unwrap())
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }

    list.iter()
        .fold(0, |acc, l| acc + l * map.get(l).unwrap_or(&0))
}
