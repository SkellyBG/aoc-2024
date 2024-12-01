use std::{collections::HashMap, iter::zip};

pub fn solve_pt1(input: &str) {
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    for line in input.split('\n') {
        let mut iter = line.split_whitespace();
        list1.push(iter.next().unwrap().parse::<i32>().unwrap());
        list2.push(iter.next().unwrap().parse::<i32>().unwrap());
    }

    list1.sort();
    list2.sort();

    let total = zip(list1, list2).fold(0, |acc, (l, r)| acc + (l - r).abs());

    println!("{}", total)
}

pub fn solve_pt2(input: &str) {
    let mut list = Vec::new();
    let mut map = HashMap::new();

    for line in input.split('\n') {
        let mut iter = line.split_whitespace();
        list.push(iter.next().unwrap().parse::<i32>().unwrap());

        map.entry(iter.next().unwrap().parse::<i32>().unwrap())
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }

    let total = list
        .iter()
        .fold(0, |acc, l| acc + l * map.get(l).unwrap_or(&0));

    println!("{}", total)
}
