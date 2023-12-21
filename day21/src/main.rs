use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    error::Error,
};

use aoc::input::parse_input_vec;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(21, input_transform)?;

    // Run parts
    println!("Part 1: {}", part1(&input, 64));
    println!("Part 2: {}", part2(&input, 26501365));

    Ok(())
}

fn part1(map: &[InputEnt], steps: usize) -> u64 {
    let max_x = map[0].len() - 1;
    let max_y = map.len() - 1;

    let mut queue = VecDeque::new();
    let mut visited = HashMap::new();

    let (x, y) = find_start(map);

    queue.push_back((x, y, 0));
    visited.insert((x, y), 0);

    while let Some((x, y, cur_steps)) = queue.pop_front() {
        let mut move_to = |nx: usize, ny: usize| {
            if map[ny][nx] != Square::Rock && !visited.contains_key(&(nx, ny)) {
                if cur_steps < steps {
                    queue.push_back((nx, ny, cur_steps + 1));
                }
                visited.insert((nx, ny), cur_steps + 1);
            }
        };

        // North
        if y > 0 {
            move_to(x, y - 1);
        }

        // East
        if x < max_x {
            move_to(x + 1, y)
        }

        // South
        if y < max_y {
            move_to(x, y + 1)
        }

        // West
        if x > 0 {
            move_to(x - 1, y)
        }
    }

    visited.iter().filter(|(_, s)| **s & 0x01 == 0).count() as u64
}

fn part2(map: &[InputEnt], steps: usize) -> u64 {
    let mut count = 0;

    let max_x = map[0].len() - 1;
    let max_y = map.len() - 1;

    let mut queue = BinaryHeap::new();
    let mut visited = HashSet::new();

    let (sx, sy) = find_start(map);

    queue.push(Scan {
        sqx: 0,
        sqy: 0,
        at: vec![(sx, sy, 0)],
        face: None,
        steps,
    });

    // println!("--- {steps} ---");

    let mut exits_cache = HashMap::new();

    while let Some(scan) = queue.pop() {
        //println!("{:?}", scan);

        let map_key = (scan.at.clone(), scan.face.clone());

        let exits = exits_cache
            .entry(map_key)
            .or_insert_with(|| calc_exits(map, &scan.at, scan.face.clone()));

        //println!("{exits:?}");

        if scan.steps < exits.max_steps {
            // Will finish in this square
            // TODO hashmap?
            let c = count_visited(map, &scan.at, scan.steps);

            // println!(
            //     "Count in {} {} from {:?} = {c}",
            //     scan.sqx, scan.sqy, scan.at
            // );

            count += c
        } else {
            // Will completely traverse this square
            let c = if steps & 0x01 == scan.steps & 0x01 {
                exits.visited_even as u64
            } else {
                exits.visited_odd as u64
            };

            // TODO REMOVE
            // let c2 = count_visited(map, &scan.at, scan.steps);
            // assert_eq!(c, c2);

            // println!(
            //     "Traverse of {} {} from {:?} = {c}",
            //     scan.sqx, scan.sqy, scan.at
            // );

            count += c;
        }

        for e in &exits.exits {
            if e.steps > scan.steps {
                continue;
            }

            let (nsqx, nsqy, at, face) = match e.face {
                Face::North => (
                    scan.sqx,
                    scan.sqy - 1,
                    e.at.iter()
                        .map(|(x, s)| (*x, max_y, *s))
                        .collect::<Vec<_>>(),
                    Face::South,
                ),
                Face::East => (
                    scan.sqx + 1,
                    scan.sqy,
                    e.at.iter()
                        .map(|(y, s)| (0usize, *y, *s))
                        .collect::<Vec<_>>(),
                    Face::West,
                ),
                Face::South => (
                    scan.sqx,
                    scan.sqy + 1,
                    e.at.iter().map(|(x, s)| (*x, 0, *s)).collect::<Vec<_>>(),
                    Face::North,
                ),
                Face::West => (
                    scan.sqx - 1,
                    scan.sqy,
                    e.at.iter()
                        .map(|(y, s)| (max_x, *y, *s))
                        .collect::<Vec<_>>(),
                    Face::East,
                ),
            };

            if !visited.contains(&(nsqx, nsqy)) {
                visited.insert((nsqx, nsqy));

                let steps = scan.steps - e.steps;
                //println!(" -> {nsqx} {nsqy} {at:?} {face:?} {steps}");

                queue.push(Scan {
                    sqx: nsqx,
                    sqy: nsqy,
                    at,
                    face: Some(face),
                    steps,
                });
            }
        }
    }

    count
}

fn find_start(input: &[InputEnt]) -> (usize, usize) {
    input
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter().enumerate().find_map(|(x, s)| {
                if *s == Square::Start {
                    Some((x, y))
                } else {
                    None
                }
            })
        })
        .expect("Unable to find start")
}

#[derive(Debug, PartialEq, Eq)]
struct Scan {
    sqx: i64,
    sqy: i64,
    at: Vec<(usize, usize, usize)>, // x,y,steps
    face: Option<Face>,
    steps: usize,
}

impl Ord for Scan {
    fn cmp(&self, other: &Self) -> Ordering {
        self.steps.cmp(&other.steps).then_with(|| {
            self.sqx
                .cmp(&other.sqx)
                .then_with(|| self.sqy.cmp(&other.sqy))
        })
    }
}

impl PartialOrd for Scan {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Face {
    North,
    East,
    South,
    West,
}

#[derive(Debug, PartialEq)]
struct Exit {
    face: Face,
    at: Vec<(usize, usize)>,
    steps: usize,
}

#[derive(Debug)]
struct Exits {
    exits: Vec<Exit>,
    max_steps: usize,
    visited_odd: usize,
    visited_even: usize,
}

fn calc_exits(
    map: &[InputEnt],
    positions: &[(usize, usize, usize)],
    from_face: Option<Face>,
) -> Exits {
    let max_x = map[0].len() - 1;
    let max_y = map.len() - 1;

    let mut queue = VecDeque::new();
    let mut visited = HashMap::new();

    positions
        .iter()
        .filter(|(_, _, s)| *s == 0)
        .for_each(|(x, y, _)| {
            queue.push_back((*x, *y, 0));
            visited.insert((*x, *y), 0);
        });

    let mut n_exit = vec![0; max_x + 1];
    let mut e_exit = vec![0; max_y + 1];
    let mut s_exit = vec![0; max_x + 1];
    let mut w_exit = vec![0; max_y + 1];

    let mut last_steps = 0;

    while let Some((x, y, cur_steps)) = queue.pop_front() {
        if cur_steps > last_steps {
            positions
                .iter()
                .filter(|(_, _, s)| *s == cur_steps)
                .for_each(|(x, y, _)| {
                    queue.push_back((*x, *y, cur_steps));
                    visited.insert((*x, *y), cur_steps);
                });

            last_steps = cur_steps;
        }

        let mut move_to = |nx: usize, ny: usize| {
            if map[ny][nx] != Square::Rock && !visited.contains_key(&(nx, ny)) {
                queue.push_back((nx, ny, cur_steps + 1));
                visited.insert((nx, ny), cur_steps + 1);
            }
        };

        let exit = |exits: &mut [usize], pos: usize| {
            if exits[pos] == 0 {
                exits[pos] = cur_steps + 1;
            }
        };

        // North
        if y > 0 {
            move_to(x, y - 1);
        } else {
            exit(&mut n_exit, x);
        }

        // East
        if x < max_x {
            move_to(x + 1, y)
        } else {
            exit(&mut e_exit, y);
        }

        // South
        if y < max_y {
            move_to(x, y + 1)
        } else {
            exit(&mut s_exit, x);
        }

        // West
        if x > 0 {
            move_to(x - 1, y)
        } else {
            exit(&mut w_exit, y);
        }
    }

    // print!("  W  N ");
    // for x in 0..=max_x {
    //     print!("{:4} ", n_exit[x])
    // }
    // println!("     E");

    // for y in 0..=max_y {
    //     print!("{:4} | ", w_exit[y]);
    //     for x in 0..=max_x {
    //         if let Some(steps) = visited.get(&(x, y)) {
    //             print!("{steps:4} ");
    //         } else {
    //             print!("{:?} ", map[y][x]);
    //         }
    //     }
    //     println!("| {:4}", e_exit[y]);
    // }

    // print!("     S ");
    // for x in 0..=max_x {
    //     print!("{:4} ", n_exit[x])
    // }
    // println!();

    let reduce_exits = |exits: Vec<usize>| -> Vec<(usize, usize)> {
        // println!("{:?}", exits);

        let mut exits = exits
            .iter()
            .enumerate()
            .filter_map(|(i, steps)| {
                if (i > 0 && exits[i - 1] == steps - 1)
                    || (i < exits.len() - 1 && exits[i + 1] == steps - 1)
                {
                    None
                } else {
                    Some((i, *steps))
                }
            })
            .collect::<Vec<_>>();

        exits.sort_by(|a, b| a.1.cmp(&b.1));

        // println!(" -> {:?}", exits);

        exits
    };

    let n_exit = reduce_exits(n_exit);
    let e_exit = reduce_exits(e_exit);
    let s_exit = reduce_exits(s_exit);
    let w_exit = reduce_exits(w_exit);

    let mut exits = Vec::new();

    let mut push_exit = |face: Face, mut exit_list: Vec<(usize, usize)>| {
        if match &from_face {
            None => true,
            Some(from_face) if *from_face != face => true,
            _ => false,
        } {
            // println!("Exits to {face:?}: {exit_list:?}");

            let min_steps = exit_list.iter().map(|(_, s)| *s).min().unwrap();

            exit_list.iter_mut().for_each(|(_, s)| *s -= min_steps);

            // println!(" -> {exit_list:?} (min {min_steps}");

            exits.push(Exit {
                face,
                at: exit_list,
                steps: min_steps,
            })
        }
    };

    push_exit(Face::North, n_exit);
    push_exit(Face::East, e_exit);
    push_exit(Face::South, s_exit);
    push_exit(Face::West, w_exit);

    let (odd, even) = visited.iter().fold((0, 0), |(odd, even), (_, s)| {
        if *s & 0x01 == 0 {
            (odd, even + 1)
        } else {
            (odd + 1, even)
        }
    });

    Exits {
        exits,
        max_steps: last_steps,
        visited_odd: odd,
        visited_even: even,
    }
}

fn count_visited(map: &[InputEnt], positions: &[(usize, usize, usize)], steps: usize) -> u64 {
    let max_x = map[0].len() - 1;
    let max_y = map.len() - 1;

    let mut queue = VecDeque::new();
    let mut visited = HashMap::new();

    positions
        .iter()
        .filter(|(_, _, s)| *s == 0)
        .for_each(|(x, y, _)| {
            queue.push_back((*x, *y, 0));
            visited.insert((*x, *y), 0);
        });

    let mut last_steps = 0;

    while let Some((x, y, cur_steps)) = queue.pop_front() {
        if cur_steps > last_steps {
            positions
                .iter()
                .filter(|(_, _, s)| *s == cur_steps)
                .for_each(|(x, y, _)| {
                    queue.push_back((*x, *y, cur_steps));
                    visited.insert((*x, *y), cur_steps);
                });

            last_steps = cur_steps;
        }

        let mut move_to = |nx: usize, ny: usize| {
            if map[ny][nx] != Square::Rock && !visited.contains_key(&(nx, ny)) {
                if cur_steps < steps {
                    queue.push_back((nx, ny, cur_steps + 1));
                }

                visited.insert((nx, ny), cur_steps + 1);
            }
        };

        // North
        if y > 0 {
            move_to(x, y - 1);
        }

        // East
        if x < max_x {
            move_to(x + 1, y)
        }

        // South
        if y < max_y {
            move_to(x, y + 1)
        }

        // West
        if x > 0 {
            move_to(x - 1, y)
        }
    }

    visited
        .iter()
        .filter(|(_, s)| **s & 0x01 == steps & 0x01)
        .count() as u64
}

// Input parsing

#[derive(Debug, PartialEq)]
enum Square {
    Start,
    Plot,
    Rock,
}

type InputEnt = Vec<Square>;

fn input_transform(line: String) -> InputEnt {
    line.chars()
        .map(|c| match c {
            'S' => Square::Start,
            '.' => Square::Plot,
            '#' => Square::Rock,
            c => panic!("Invalid char {c} in {line}"),
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use aoc::input::parse_test_vec;

    use super::*;

    const EXAMPLE1: &str = "\
...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";

    #[test]
    fn test1() {
        let input = parse_test_vec(EXAMPLE1, input_transform).unwrap();

        assert_eq!(part1(&input, 6), 16);

        assert_eq!(part2(&input, 6), 16);
        assert_eq!(part2(&input, 7), 22);
        assert_eq!(part2(&input, 10), 50);
        assert_eq!(part2(&input, 50), 1_594);
        assert_eq!(part2(&input, 100), 6_536);
        assert_eq!(part2(&input, 500), 167_004);
        assert_eq!(part2(&input, 1000), 668_697);
        assert_eq!(part2(&input, 5000), 16_733_044);
    }
}

// 0.....09890 =  6,  4e   2o
// 90...###7#. =  3,  1e   2o
// 8###.##56#0 =  4,  3e   1o
// 76#4#234#89 =  8,  5e   3o
// 6543#1#5678 =  9,  4e   5o
// 7##21S####9 =  5,  2e   3o
// 8##32#678#0 =  7,  5e   2o
// 7654345##.. =  7,  3e   4o
// 8##5#5####. =  3,  1e   2o
// 9##67##.##. =  3,  1e   2o
// 0987890.... =  7,  4e   3o
//             = 62, 33e, 29o

// ........... ........... ...........
// .....###.#. .....###.#. .....###.#.
// .###.##..#. .###.##..#. .###.##..#.
// ..#.#...#.. ..#.#...#.. ..#.#...#..
// ....#.#.... ....#.#.... ....#.#....
// .##..S####. .##..S####. .##..S####.
// .##..#...#. .##..#...#. .##..#...#.
// .......##.. .......##.. .......##..
// .##.#.####. .##.#.####. .##.#.####.
// .##..##.##. .##..##.##. .##..##.##.
// ........... .......090. ...........

// ........... 0.....09890 ...........
// .....###.#0 90...###7#. .....###.#.
// .###.##..#9 8###.##56#0 .###.##..#.
// ..#.#...#98 76#4#234#89 0.#.#...#..
// ....#.#0987 6543#1#5678 90..#.#....
// .##..S####8 7##21S####9 0##..S####.
// .##..#...#9 8##32#678#0 .##..#...#.
// .......##98 7654345##.. .......##..
// .##.#.####9 8##5#5####. .##.#.####.
// .##..##.##0 9##67##.##. .##..##.##.
// ........... 0987890.... ...........

// ........... .09890..... ...........
// .....###.#. ..090###.#. .....###.#.
// .###.##..#. .###.##..#. .###.##..#.
// ..#.#...#.. ..#.#...#.. ..#.#...#..
// ....#.#.... ....#.#.... ....#.#....
// .##..S####. .##..S####. .##..S####.
// .##..#...#. .##..#...#. .##..#...#.
// .......##.. .......##.. .......##..
// .##.#.####. .##.#.####. .##.#.####.
// .##..##.##. .##..##.##. .##..##.##.
// ........... ........... ...........
