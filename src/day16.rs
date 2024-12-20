use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::hash::Hash;

use aoc_runner_derive::{aoc, aoc_generator};

use crate::grid::Grid;

#[aoc_generator(day16)]
fn parse(input: &str) -> Grid {
    Grid::new(input)
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Debug)]
struct Raindeer {
    position: (usize, usize),
    direction: Direction,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Debug)]
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

    let mut pq: BinaryHeap<Reverse<State>> = BinaryHeap::new();

    let mut visited: HashSet<Raindeer> = HashSet::new();

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
fn part2(input: &Grid) -> usize {
    let s = input.find(b'S').unwrap();
    let e = input.find(b'E').unwrap();

    let raindeer = Raindeer {
        position: s,
        direction: Direction::Right,
    };

    let mut pq: BinaryHeap<Reverse<(State, Raindeer)>> = BinaryHeap::new();

    let mut visited: HashMap<Raindeer, i32> = HashMap::new();

    let mut parent: HashMap<Raindeer, Vec<Raindeer>> = HashMap::new();

    pq.push(Reverse((State { score: 0, raindeer }, raindeer)));

    visited.insert(raindeer, 0);

    let mut score = None;

    loop {
        let state = pq.pop().unwrap().0;

        if let Some(score) = score {
            if state.0.score > score {
                break;
            }
        }

        // println!("{:?}", state);

        if state.0.raindeer.position == e {
            score = Some(state.0.score);
        }

        match visited.entry(state.0.raindeer) {
            std::collections::hash_map::Entry::Occupied(occupied_entry) => {
                if *occupied_entry.get() == state.0.score {
                    parent
                        .entry(state.0.raindeer)
                        .and_modify(|l| l.push(state.1))
                        .or_insert(vec![state.1]);
                }
            }
            std::collections::hash_map::Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(state.0.score);

                parent
                    .entry(state.0.raindeer)
                    .and_modify(|l| l.push(state.1))
                    .or_insert(vec![state.1]);
            }
        }

        let (state_move, state_left, state_right) = match state.0.raindeer.direction {
            Direction::Up => {
                let new_state_move = State {
                    score: state.0.score + 1,
                    raindeer: Raindeer {
                        position: (state.0.raindeer.position.0 - 1, state.0.raindeer.position.1),
                        direction: Direction::Up,
                    },
                };

                let new_state_left = State {
                    score: state.0.score + 1000,
                    raindeer: Raindeer {
                        position: state.0.raindeer.position,
                        direction: Direction::Left,
                    },
                };

                let new_state_right = State {
                    score: state.0.score + 1000,
                    raindeer: Raindeer {
                        position: state.0.raindeer.position,
                        direction: Direction::Right,
                    },
                };

                (new_state_move, new_state_left, new_state_right)
            }
            Direction::Down => {
                let new_state_move = State {
                    score: state.0.score + 1,
                    raindeer: Raindeer {
                        position: (state.0.raindeer.position.0 + 1, state.0.raindeer.position.1),
                        direction: Direction::Down,
                    },
                };

                let new_state_left = State {
                    score: state.0.score + 1000,
                    raindeer: Raindeer {
                        position: state.0.raindeer.position,
                        direction: Direction::Right,
                    },
                };

                let new_state_right = State {
                    score: state.0.score + 1000,
                    raindeer: Raindeer {
                        position: state.0.raindeer.position,
                        direction: Direction::Left,
                    },
                };

                (new_state_move, new_state_left, new_state_right)
            }
            Direction::Right => {
                let new_state_move = State {
                    score: state.0.score + 1,
                    raindeer: Raindeer {
                        position: (state.0.raindeer.position.0, state.0.raindeer.position.1 + 1),
                        direction: Direction::Right,
                    },
                };

                let new_state_left = State {
                    score: state.0.score + 1000,
                    raindeer: Raindeer {
                        position: state.0.raindeer.position,
                        direction: Direction::Up,
                    },
                };

                let new_state_right = State {
                    score: state.0.score + 1000,
                    raindeer: Raindeer {
                        position: state.0.raindeer.position,
                        direction: Direction::Down,
                    },
                };

                (new_state_move, new_state_left, new_state_right)
            }
            Direction::Left => {
                let new_state_move = State {
                    score: state.0.score + 1,
                    raindeer: Raindeer {
                        position: (state.0.raindeer.position.0, state.0.raindeer.position.1 - 1),
                        direction: Direction::Left,
                    },
                };

                let new_state_left = State {
                    score: state.0.score + 1000,
                    raindeer: Raindeer {
                        position: state.0.raindeer.position,
                        direction: Direction::Down,
                    },
                };

                let new_state_right = State {
                    score: state.0.score + 1000,
                    raindeer: Raindeer {
                        position: state.0.raindeer.position,
                        direction: Direction::Up,
                    },
                };

                (new_state_move, new_state_left, new_state_right)
            }
        };

        if input.cells[state_move.raindeer.position.0][state_move.raindeer.position.1] != b'#' {
            pq.push(Reverse((state_move, state.0.raindeer)));
        }

        if !visited.contains_key(&state_left.raindeer) {
            pq.push(Reverse((state_left, state.0.raindeer)));
        }

        if !visited.contains_key(&state_right.raindeer) {
            pq.push(Reverse((state_right, state.0.raindeer)));
        }
    }

    bfs(parent, e, input)
}

fn bfs(parent: HashMap<Raindeer, Vec<Raindeer>>, e: (usize, usize), grid: &Grid) -> usize {
    let mut q = VecDeque::new();

    let mut visited = HashSet::new();
    let mut res = HashSet::new();

    q.push_back(Raindeer {
        position: e,
        direction: Direction::Up,
    });
    q.push_back(Raindeer {
        position: e,
        direction: Direction::Right,
    });
    visited.insert(Raindeer {
        position: e,
        direction: Direction::Up,
    });
    visited.insert(Raindeer {
        position: e,
        direction: Direction::Right,
    });

    while let Some(front) = q.pop_front() {
        for neighbor in parent.get(&front).unwrap_or(&Vec::new()) {
            if visited.contains(&neighbor) {
                continue;
            }
            visited.insert(*neighbor);
            res.insert(neighbor.position);
            q.push_back(*neighbor);
        }
    }

    for (x, row) in grid.cells.iter().enumerate() {
        for (y, cell) in row.iter().enumerate() {
            if res.contains(&(x, y)) {
                print!("O");
            } else {
                print!("{}", *cell as char);
            }
        }
        println!();
    }

    res.len() + 1
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
const EXAMPLE2: &str = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 7036);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 45);
        assert_eq!(part2(&parse(EXAMPLE2)), 64);
    }
}
