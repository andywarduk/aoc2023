use std::{
    collections::HashMap,
    error::Error,
    hash::{DefaultHasher, Hash, Hasher},
};

use aoc::input::parse_input_vec;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(14, input_transform)?;

    // Run parts
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

fn part1(map: &[InputEnt]) -> u64 {
    let mut map = map.to_vec();

    // Roll rocks north
    roll(&mut map, Dir::N);

    // Calculate load
    calc_load(&map)
}

fn part2(map: &[InputEnt]) -> u64 {
    let mut map = map.to_vec();

    let mut hashes = HashMap::new();

    let mut end_iter = 1_000_000_000;
    let mut loop_found = false;
    let mut i = 0;

    loop {
        // Finished?
        if i >= end_iter {
            break;
        }

        // Roll the rocks
        roll(&mut map, Dir::N);
        roll(&mut map, Dir::W);
        roll(&mut map, Dir::S);
        roll(&mut map, Dir::E);

        if !loop_found {
            // Hash the map
            let mut hasher = DefaultHasher::new();
            map.hash(&mut hasher);
            let new_hash = hasher.finish();

            // Already got this state?
            if let Some(start) = hashes.get(&new_hash) {
                // Found loop - calculate extra iterations required to match the state at 1,000,000,000
                end_iter = i + ((1_000_000_000 - start) % (i - start));
                loop_found = true;
            } else {
                hashes.insert(new_hash, i);
            }
        }

        i += 1;
    }

    // Calculate the load
    calc_load(&map)
}

fn roll(map: &mut [InputEnt], dir: Dir) {
    match dir {
        Dir::N => (0..map[0].len()).for_each(|x| {
            (1..map.len()).for_each(|y| {
                if map[y][x] == State::Rock {
                    let lx = x;
                    roll_rock(map, x, y, (0..y).rev().map(move |ly| (lx, ly)));
                }
            });
        }),
        Dir::S => (0..map[0].len()).for_each(|x| {
            (0..(map.len() - 1)).rev().for_each(|y| {
                if map[y][x] == State::Rock {
                    let lx = x;
                    roll_rock(map, x, y, ((y + 1)..map.len()).map(move |ly| (lx, ly)));
                }
            });
        }),
        Dir::E => (0..map.len()).for_each(|y| {
            (0..(map[0].len() - 1)).rev().for_each(|x| {
                if map[y][x] == State::Rock {
                    let ly = y;
                    roll_rock(map, x, y, ((x + 1)..map[0].len()).map(move |lx| (lx, ly)));
                }
            });
        }),
        Dir::W => (0..map.len()).for_each(|y| {
            (1..map[0].len()).for_each(|x| {
                if map[y][x] == State::Rock {
                    let ly = y;
                    roll_rock(map, x, y, (0..x).rev().map(move |lx| (lx, ly)));
                }
            });
        }),
    }
}

fn roll_rock(
    map: &mut [InputEnt],
    x: usize,
    y: usize,
    pos_iter: impl Iterator<Item = (usize, usize)>,
) {
    map[y][x] = State::Empty;

    let (mut rx, mut ry) = (x, y);

    for (cx, cy) in pos_iter {
        if map[cy][cx] == State::Empty {
            (rx, ry) = (cx, cy)
        } else {
            break;
        }
    }

    map[ry][rx] = State::Rock;
}

fn calc_load(map: &[InputEnt]) -> u64 {
    // Calculate load
    map.iter()
        .rev()
        .enumerate()
        .map(|(mult, row)| {
            row.iter().filter(|p| **p == State::Rock).count() as u64 * (mult as u64 + 1)
        })
        .sum()
}

#[derive(Debug, PartialEq, Clone, Hash)]
enum State {
    Empty,
    Rock,
    Cube,
}

#[derive(Debug)]
enum Dir {
    N,
    E,
    S,
    W,
}

// Input parsing

type InputEnt = Vec<State>;

fn input_transform(line: String) -> InputEnt {
    line.chars()
        .map(|c| match c {
            '.' => State::Empty,
            '#' => State::Cube,
            'O' => State::Rock,
            _ => panic!("Invalid char"),
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use aoc::input::parse_test_vec;

    use super::*;

    const EXAMPLE1: &str = "\
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    #[test]
    fn test1() {
        let input = parse_test_vec(EXAMPLE1, input_transform).unwrap();
        assert_eq!(part1(&input), 136);
        assert_eq!(part2(&input), 64);
    }

    #[test]
    fn test2() {
        let mut map = parse_test_vec(EXAMPLE1, input_transform).unwrap();

        for i in 0..3 {
            roll(&mut map, Dir::N);
            roll(&mut map, Dir::W);
            roll(&mut map, Dir::S);
            roll(&mut map, Dir::E);

            let expected = match i {
                0 => {
                    "\
.....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#...."
                }
                1 => {
                    "\
.....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O"
                }
                2 => {
                    "\
.....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O"
                }
                _ => unreachable!(),
            };

            let expected_map = parse_test_vec(expected, input_transform).unwrap();

            assert_eq!(map, expected_map, "Map incorrect")
        }
    }
}
