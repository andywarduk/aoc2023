use std::{
    cmp::max,
    collections::{HashSet, VecDeque},
    error::Error,
};

use aoc::input::parse_input_vec;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(16, input_transform)?;

    // Run parts
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

fn part1(map: &[InputEnt]) -> u64 {
    energise(map, 0, 0, Dir::E)
}

fn part2(map: &[InputEnt]) -> u64 {
    let mut result = 0;

    // Top and bottom rows
    for x in 0..(map[0].len()) {
        result = max(result, energise(map, x, 0, Dir::S));
        result = max(result, energise(map, x, map.len() - 1, Dir::N));
    }

    // Left and Right columns
    for y in 0..(map.len()) {
        result = max(result, energise(map, 0, y, Dir::E));
        result = max(result, energise(map, map[0].len() - 1, y, Dir::W));
    }

    result
}

fn energise(map: &[InputEnt], x: usize, y: usize, dir: Dir) -> u64 {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    // Add initial position and direction
    queue.push_back((x, y, dir));

    // Get next queue entry
    while let Some((x, y, dir)) = queue.pop_front() {
        // Build hash set entry
        let visited_ent = (x, y, dir.clone());

        // Already been here travelling in this direction?
        if visited.contains(&visited_ent) {
            continue;
        }

        // Add to visited
        visited.insert(visited_ent);

        match map[y][x] {
            State::Empty => {
                // Continue on this path
                if let Some((x, y)) = add_dir(map, x, y, &dir) {
                    queue.push_back((x, y, dir));
                }
            }
            State::MirrorNESW => {
                // Work out new direction
                let new_dir = match dir {
                    Dir::N => Dir::E,
                    Dir::E => Dir::N,
                    Dir::S => Dir::W,
                    Dir::W => Dir::S,
                };

                // Move in new direction
                if let Some((x, y)) = add_dir(map, x, y, &new_dir) {
                    queue.push_back((x, y, new_dir));
                }
            }
            State::MirrorNWSE => {
                // Work out new direction
                let new_dir = match dir {
                    Dir::N => Dir::W,
                    Dir::E => Dir::S,
                    Dir::S => Dir::E,
                    Dir::W => Dir::N,
                };

                // Move in new direction
                if let Some((x, y)) = add_dir(map, x, y, &new_dir) {
                    queue.push_back((x, y, new_dir));
                }
            }
            State::SplitterHoriz => match dir {
                Dir::E | Dir::W => {
                    // Continue on this path
                    if let Some((x, y)) = add_dir(map, x, y, &dir) {
                        queue.push_back((x, y, dir));
                    }
                }
                Dir::S | Dir::N => {
                    // Split east
                    let dir1 = Dir::E;

                    if let Some((x, y)) = add_dir(map, x, y, &dir1) {
                        queue.push_back((x, y, dir1));
                    }

                    // Split west
                    let dir2 = Dir::W;

                    if let Some((x, y)) = add_dir(map, x, y, &dir2) {
                        queue.push_back((x, y, dir2));
                    }
                }
            },
            State::SplitterVert => match dir {
                Dir::S | Dir::N => {
                    // Continue on this path
                    if let Some((x, y)) = add_dir(map, x, y, &dir) {
                        queue.push_back((x, y, dir));
                    }
                }
                Dir::E | Dir::W => {
                    // Split north
                    let dir1 = Dir::N;

                    if let Some((x, y)) = add_dir(map, x, y, &dir1) {
                        queue.push_back((x, y, dir1));
                    }

                    // Split south
                    let dir2 = Dir::S;

                    if let Some((x, y)) = add_dir(map, x, y, &dir2) {
                        queue.push_back((x, y, dir2));
                    }
                }
            },
        };
    }

    // Build set of visited locations
    let visited_set = visited
        .iter()
        .map(|(x, y, _)| (x, y))
        .collect::<HashSet<_>>();

    visited_set.len() as u64
}

fn add_dir(map: &[InputEnt], x: usize, y: usize, dir: &Dir) -> Option<(usize, usize)> {
    // Get movement
    let (xadd, yadd) = dir.movement();

    // Work out new position
    let new_x = x as isize + xadd;
    let new_y = y as isize + yadd;

    // Bounds check new position
    if new_x >= 0 && (new_x as usize) < map[0].len() && new_y >= 0 && (new_y as usize) < map.len() {
        // Within map bounds
        Some((new_x as usize, new_y as usize))
    } else {
        // Outside map
        None
    }
}

#[derive(Debug)]
enum State {
    Empty,
    MirrorNESW,
    MirrorNWSE,
    SplitterHoriz,
    SplitterVert,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Dir {
    N,
    E,
    S,
    W,
}

impl Dir {
    fn movement(&self) -> (isize, isize) {
        match self {
            Dir::N => (0, -1),
            Dir::E => (1, 0),
            Dir::S => (0, 1),
            Dir::W => (-1, 0),
        }
    }
}

// Input parsing

type InputEnt = Vec<State>;

fn input_transform(line: String) -> InputEnt {
    line.chars()
        .map(|c| match c {
            '.' => State::Empty,
            '/' => State::MirrorNESW,
            '\\' => State::MirrorNWSE,
            '-' => State::SplitterHoriz,
            '|' => State::SplitterVert,
            _ => panic!("Unvalid character"),
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use aoc::input::parse_test_vec;

    use super::*;

    const EXAMPLE1: &str = "\
.|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....";

    #[test]
    fn test1() {
        let input = parse_test_vec(EXAMPLE1, input_transform).unwrap();
        assert_eq!(part1(&input), 46);
        assert_eq!(part2(&input), 51);
    }
}
