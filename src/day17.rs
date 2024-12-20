use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Adv(i64),
    Bxl(i64),
    Bst(i64),
    Jnz(i64),
    Bxc(()),
    Out(i64),
    Bdv(i64),
    Cdv(i64),
}

#[derive(Debug, Clone, Copy)]
struct Computer {
    r_a: i64,
    r_b: i64,
    r_c: i64,
    i_ptr: usize,
}

#[aoc_generator(day17)]
fn parse(input: &str) -> (Computer, Vec<Instruction>) {
    let mut lines = input.split('\n');
    let computer = Computer {
        r_a: lines
            .next()
            .unwrap()
            .split(' ')
            .last()
            .unwrap()
            .parse()
            .unwrap(),
        r_b: lines
            .next()
            .unwrap()
            .split(' ')
            .last()
            .unwrap()
            .parse()
            .unwrap(),
        r_c: lines
            .next()
            .unwrap()
            .split(' ')
            .last()
            .unwrap()
            .parse()
            .unwrap(),
        i_ptr: 0,
    };

    lines.next();

    let instructions = lines
        .next()
        .unwrap()
        .split(' ')
        .last()
        .unwrap()
        .split(',')
        .chunks(2)
        .into_iter()
        .map(|chunk| {
            let mut iter = chunk;
            let op = iter.next().unwrap();
            let arg = iter.next().unwrap().parse().unwrap();
            match op {
                "0" => Instruction::Adv(arg),
                "1" => Instruction::Bxl(arg),
                "2" => Instruction::Bst(arg),
                "3" => Instruction::Jnz(arg),
                "4" => Instruction::Bxc(()),
                "5" => Instruction::Out(arg),
                "6" => Instruction::Bdv(arg),
                "7" => Instruction::Cdv(arg),
                _ => panic!("Unknown instruction: {}", op),
            }
        })
        .collect();

    (computer, instructions)
}

fn combo(v: i64, computer: &Computer) -> i64 {
    match v {
        0 => 0,
        1 => 1,
        2 => 2,
        3 => 3,
        4 => computer.r_a,
        5 => computer.r_b,
        6 => computer.r_c,
        _ => panic!("Unknown value: {}", v),
    }
}

#[aoc(day17, part1)]
fn part1(input: &(Computer, Vec<Instruction>)) -> String {
    let (computer, instructions) = input;

    let mut computer = *computer;

    let mut res = Vec::new();

    loop {
        match instructions.get(computer.i_ptr) {
            None => break,
            Some(instruction) => match *instruction {
                Instruction::Adv(v) => {
                    let res = computer.r_a / 2_i64.pow(combo(v, &computer).try_into().unwrap());

                    computer.r_a = res;
                }
                Instruction::Bxl(v) => {
                    computer.r_b = v ^ computer.r_b;
                }
                Instruction::Bst(v) => {
                    computer.r_b = combo(v, &computer) % 8;
                }
                Instruction::Jnz(v) => {
                    if computer.r_a != 0 {
                        computer.i_ptr = (v / 2) as usize;
                        continue;
                    }
                }
                Instruction::Bxc(_) => {
                    computer.r_b = computer.r_b ^ computer.r_c;
                }
                Instruction::Out(v) => res.push(combo(v, &computer) % 8),
                Instruction::Bdv(v) => {
                    let res = computer.r_a / 2_i64.pow(combo(v, &computer).try_into().unwrap());

                    computer.r_b = res;
                }
                Instruction::Cdv(v) => {
                    let res = computer.r_a / 2_i64.pow(combo(v, &computer).try_into().unwrap());

                    computer.r_c = res;
                }
            },
        }

        computer.i_ptr += 1;
    }

    res.into_iter().map(|c| c.to_string()).join(",")
}

#[aoc(day17, part2)]
fn part2(input: &(Computer, Vec<Instruction>)) -> String {
    let (computer, instructions) = input;

    let mut computer = *computer;

    let mut res = Vec::new();

    let digits = vec![5, 3, 2, 2, 4, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1];
    computer.r_a = digits.iter().fold(0, |acc, &d| acc * 8 + d);

    loop {
        match instructions.get(computer.i_ptr) {
            None => break,
            Some(instruction) => match *instruction {
                Instruction::Adv(v) => {
                    let res = computer.r_a / 2_i64.pow(combo(v, &computer).try_into().unwrap());

                    computer.r_a = res;
                }
                Instruction::Bxl(v) => {
                    computer.r_b = v ^ computer.r_b;
                }
                Instruction::Bst(v) => {
                    computer.r_b = combo(v, &computer) % 8;
                }
                Instruction::Jnz(v) => {
                    if computer.r_a != 0 {
                        computer.i_ptr = (v / 2) as usize;
                        continue;
                    }
                }
                Instruction::Bxc(_) => {
                    computer.r_b = computer.r_b ^ computer.r_c;
                }
                Instruction::Out(v) => res.push(combo(v, &computer) % 8),
                Instruction::Bdv(v) => {
                    let res = computer.r_a / 2_i64.pow(combo(v, &computer).try_into().unwrap());

                    computer.r_b = res;
                }
                Instruction::Cdv(v) => {
                    let res = computer.r_a / 2_i64.pow(combo(v, &computer).try_into().unwrap());

                    computer.r_c = res;
                }
            },
        }

        computer.i_ptr += 1;
    }

    res.into_iter().map(|c| c.to_string()).join(",")
}

#[cfg(test)]
const EXAMPLE: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), "<RESULT>");
    }
}
