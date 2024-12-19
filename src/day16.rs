use std::cmp::Reverse;

use aoc_runner_derive::{aoc, aoc_generator};

use crate::grid::Grid;

#[aoc_generator(day16)]
fn parse(input: &str) -> Grid {
    Grid::new(input)
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
struct Raindeer {
    position: (usize, usize),
    direction: Direction,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
struct State {
    score: i32,
    raindeer: Raindeer,
}

#[aoc(day16, part1)]
fn part1(input: &Grid) -> i32 {
    let s = input.find(b'S').unwrap();
    let e = input.find(b'E').unwrap();

    let raindeer = Raindeer {
        position: s,
        direction: Direction::Right,
    };

    let mut pq: std::collections::BinaryHeap<Reverse<State>> = std::collections::BinaryHeap::new();

    let mut visited: std::collections::HashSet<Raindeer> = std::collections::HashSet::new();

    pq.push(Reverse(State { score: 0, raindeer }));

    visited.insert(raindeer);

    loop {
        let state = pq.pop().unwrap().0;

        if state.raindeer.position == e {
            return state.score;
        }

        let (state_move, state_left, state_right) = match state.raindeer.direction {
            Direction::Up => {
                let new_state_move = State {
                    score: state.score + 1,
                    raindeer: Raindeer {
                        position: (state.raindeer.position.0 - 1, state.raindeer.position.1),
                        direction: Direction::Up,
                    },
                };

                let new_state_left = State {
                    score: state.score + 1000,
                    raindeer: Raindeer {
                        position: state.raindeer.position,
                        direction: Direction::Left,
                    },
                };

                let new_state_right = State {
                    score: state.score + 1000,
                    raindeer: Raindeer {
                        position: state.raindeer.position,
                        direction: Direction::Right,
                    },
                };

                (new_state_move, new_state_left, new_state_right)
            }
            Direction::Down => {
                let new_state_move = State {
                    score: state.score + 1,
                    raindeer: Raindeer {
                        position: (state.raindeer.position.0 + 1, state.raindeer.position.1),
                        direction: Direction::Down,
                    },
                };

                let new_state_left = State {
                    score: state.score + 1000,
                    raindeer: Raindeer {
                        position: state.raindeer.position,
                        direction: Direction::Right,
                    },
                };

                let new_state_right = State {
                    score: state.score + 1000,
                    raindeer: Raindeer {
                        position: state.raindeer.position,
                        direction: Direction::Left,
                    },
                };

                (new_state_move, new_state_left, new_state_right)
            }
            Direction::Right => {
                let new_state_move = State {
                    score: state.score + 1,
                    raindeer: Raindeer {
                        position: (state.raindeer.position.0, state.raindeer.position.1 + 1),
                        direction: Direction::Right,
                    },
                };

                let new_state_left = State {
                    score: state.score + 1000,
                    raindeer: Raindeer {
                        position: state.raindeer.position,
                        direction: Direction::Up,
                    },
                };

                let new_state_right = State {
                    score: state.score + 1000,
                    raindeer: Raindeer {
                        position: state.raindeer.position,
                        direction: Direction::Down,
                    },
                };

                (new_state_move, new_state_left, new_state_right)
            }
            Direction::Left => {
                let new_state_move = State {
                    score: state.score + 1,
                    raindeer: Raindeer {
                        position: (state.raindeer.position.0, state.raindeer.position.1 - 1),
                        direction: Direction::Left,
                    },
                };

                let new_state_left = State {
                    score: state.score + 1000,
                    raindeer: Raindeer {
                        position: state.raindeer.position,
                        direction: Direction::Down,
                    },
                };

                let new_state_right = State {
                    score: state.score + 1000,
                    raindeer: Raindeer {
                        position: state.raindeer.position,
                        direction: Direction::Up,
                    },
                };

                (new_state_move, new_state_left, new_state_right)
            }
        };

        if !visited.contains(&state_move.raindeer)
            && input.cells[state_move.raindeer.position.0][state_move.raindeer.position.1] != b'#'
        {
            pq.push(Reverse(state_move));
            visited.insert(state_move.raindeer);
        }

        if !visited.contains(&state_left.raindeer) {
            pq.push(Reverse(state_left));
            visited.insert(state_left.raindeer);
        }

        if !visited.contains(&state_right.raindeer) {
            pq.push(Reverse(state_right));
            visited.insert(state_right.raindeer);
        }
    }
}

#[aoc(day16, part2)]
fn part2(input: &Grid) -> i32 {
    todo!()
}

#[cfg(test)]
const EXAMPLE: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 7036);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 40);
    }
}
