use std::{fs::File, io::Read};

mod days;

fn main() {
    let mut buf = String::new();

    File::open("input/day01.txt")
        .unwrap()
        .read_to_string(&mut buf)
        .unwrap();

    days::day01::solve_pt2(&buf);
}
