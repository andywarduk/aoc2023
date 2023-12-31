use std::{collections::HashSet, error::Error};

use aoc::input::parse_input_vec;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let mut input = parse_input_vec(10, input_transform)?;
    let (x, y) = find_start(&mut input);

    // Run parts
    println!("Part 1: {}", part1(&input, x, y));
    println!("Part 2: {}", part2(&input, x, y));

    Ok(())
}

fn find_start(map: &mut [MapRow]) -> (usize, usize) {
    // Find start
    let (x, y) = map
        .iter()
        .enumerate()
        .find_map(|(y, l)| {
            l.iter()
                .enumerate()
                .find_map(|(x, c)| if *c == Pipe::Start { Some(x) } else { None })
                .map(|x| (x, y))
        })
        .unwrap();

    let mut dirs = Vec::new();

    // Which directions?
    match map[y - 1][x] {
        Pipe::NS | Pipe::SE | Pipe::SW => dirs.push(Dir::N),
        _ => (),
    }
    match map[y + 1][x] {
        Pipe::NS | Pipe::NE | Pipe::NW => dirs.push(Dir::S),
        _ => (),
    }
    match map[y][x - 1] {
        Pipe::EW | Pipe::SE | Pipe::NE => dirs.push(Dir::W),
        _ => (),
    }
    match map[y][x + 1] {
        Pipe::EW | Pipe::SW | Pipe::NW => dirs.push(Dir::E),
        _ => (),
    }

    dirs.sort();

    let start_pipe = match (dirs[0], dirs[1]) {
        (Dir::N, Dir::E) => Pipe::NE,
        (Dir::N, Dir::S) => Pipe::NS,
        (Dir::N, Dir::W) => Pipe::NW,
        (Dir::E, Dir::S) => Pipe::SE,
        (Dir::E, Dir::W) => Pipe::EW,
        (Dir::S, Dir::W) => Pipe::SW,
        _ => panic!("Unable to find start pipe"),
    };

    map[y][x] = start_pipe;

    (x, y)
}

fn start_dir(map: &[MapRow], x: usize, y: usize) -> Dir {
    // Choose a start direction
    match map[y][x] {
        Pipe::NS | Pipe::NE | Pipe::NW => Dir::S,
        Pipe::EW | Pipe::SE => Dir::W,
        Pipe::SW => Dir::E,
        _ => panic!("Invalid start pipe"),
    }
}

fn part1(map: &[MapRow], start_x: usize, start_y: usize) -> u64 {
    let mut x = start_x;
    let mut y = start_y;
    let mut dir_from = start_dir(map, x, y);

    // Walk the loop
    let mut steps = 0;

    loop {
        dir_from = map[y][x].next_dir(dir_from);

        (x, y) = match dir_from {
            Dir::N => (x, y - 1),
            Dir::S => (x, y + 1),
            Dir::E => (x + 1, y),
            Dir::W => (x - 1, y),
        };

        steps += 1;

        if x == start_x && y == start_y {
            break;
        }
    }

    steps / 2
}

fn part2(map: &[MapRow], start_x: usize, start_y: usize) -> u64 {
    let mut x = start_x;
    let mut y = start_y;
    let mut dir_from = start_dir(map, x, y);

    // Walk the loop
    let mut visited = HashSet::new();

    loop {
        visited.insert((x, y));

        dir_from = map[y][x].next_dir(dir_from);

        (x, y) = match dir_from {
            Dir::N => (x, y - 1),
            Dir::S => (x, y + 1),
            Dir::E => (x + 1, y),
            Dir::W => (x - 1, y),
        };

        if x == start_x && y == start_y {
            break;
        }
    }

    // Called when a pipe is being crossed
    let cross_pipe = |pipe_count: &mut usize, in_dir: &mut Option<Dir>| {
        *pipe_count += 1;
        *in_dir = None
    };

    // Called when coming in in a given direction
    let in_out = |dir: Dir, pipe_count: &mut usize, in_dir: &mut Option<Dir>| match in_dir {
        Some(cur_dir) => {
            if *cur_dir == dir {
                // In and out same direction
                *in_dir = None
            } else {
                // In and out in opposite directions
                cross_pipe(pipe_count, in_dir)
            }
        }
        None => {
            // In in a direction
            *in_dir = Some(dir)
        }
    };

    // Find contained squares
    map.iter().enumerate().fold(0, |acc, (y, l)| {
        let mut pipe_count = 0;
        let mut in_dir = None;
        let mut contained = 0;

        for (x, c) in l.iter().enumerate() {
            // Square part of the visited pipe?
            if visited.contains(&(x, y)) {
                match c {
                    Pipe::NS => cross_pipe(&mut pipe_count, &mut in_dir),
                    Pipe::NE | Pipe::NW => in_out(Dir::N, &mut pipe_count, &mut in_dir),
                    Pipe::SW | Pipe::SE => in_out(Dir::S, &mut pipe_count, &mut in_dir),
                    Pipe::EW => (),
                    _ => panic!("Invalid pipe"),
                }
            } else if pipe_count & 1 == 1 {
                // Odd number of pipes crossed means this unvisited square is inside the loop
                contained += 1;
            }
        }

        acc + contained
    })
}

#[derive(Debug, PartialEq)]
enum Pipe {
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
    Ground,
    Start,
}

impl Pipe {
    fn next_dir(&self, dir: Dir) -> Dir {
        match self {
            Pipe::NS => match dir {
                Dir::S => Dir::S,
                Dir::N => Dir::N,
                _ => panic!("Invalid direction {dir:?} for {self:?}"),
            },
            Pipe::EW => match dir {
                Dir::E => Dir::E,
                Dir::W => Dir::W,
                _ => panic!("Invalid direction {dir:?} for {self:?}"),
            },
            Pipe::NE => match dir {
                Dir::S => Dir::E,
                Dir::W => Dir::N,
                _ => panic!("Invalid direction {dir:?} for {self:?}"),
            },
            Pipe::NW => match dir {
                Dir::S => Dir::W,
                Dir::E => Dir::N,
                _ => panic!("Invalid direction {dir:?} for {self:?}"),
            },
            Pipe::SW => match dir {
                Dir::E => Dir::S,
                Dir::N => Dir::W,
                _ => panic!("Invalid direction {dir:?} for {self:?}"),
            },
            Pipe::SE => match dir {
                Dir::N => Dir::E,
                Dir::W => Dir::S,
                _ => panic!("Invalid direction {dir:?} for {self:?}"),
            },
            _ => panic!("Invalid pipe"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Dir {
    N,
    E,
    S,
    W,
}

// Input parsing

type MapRow = Vec<Pipe>;

fn input_transform(line: String) -> MapRow {
    line.chars()
        .map(|c| match c {
            '|' => Pipe::NS,
            '-' => Pipe::EW,
            'L' => Pipe::NE,
            'J' => Pipe::NW,
            '7' => Pipe::SW,
            'F' => Pipe::SE,
            '.' => Pipe::Ground,
            'S' => Pipe::Start,
            _ => panic!("Invalid char"),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use aoc::input::parse_test_vec;

    use super::*;

    const EXAMPLE1: &str = "\
.....
.S-7.
.|.|.
.L-J.
.....";

    const EXAMPLE2: &str = "\
-L|F7
7S-7|
L|7||
-L-J|
L|-JF";

    #[test]
    fn test1() {
        let mut input = parse_test_vec(EXAMPLE1, input_transform).unwrap();
        let (x, y) = find_start(&mut input);

        assert_eq!(part1(&input, x, y), 4);
    }

    #[test]
    fn test2() {
        let mut input = parse_test_vec(EXAMPLE2, input_transform).unwrap();
        let (x, y) = find_start(&mut input);

        assert_eq!(part1(&input, x, y), 4);
    }

    #[test]
    fn test3() {
        let mut input = parse_test_vec(EXAMPLE2, input_transform).unwrap();
        let (x, y) = find_start(&mut input);

        assert_eq!(part2(&input, x, y), 1);
    }
}
