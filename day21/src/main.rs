use std::{
    collections::{HashMap, VecDeque},
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
    let dim = map.len();
    let div = (steps / dim) as i64;
    let remainder = steps % dim;

    // Get 3 points in the quadratic equation
    let s0 = remainder;
    let s1 = s0 + dim;
    let s2 = s1 + dim;

    let p0 = p2solve(map, s0);
    let p1 = p2solve(map, s1);
    let p2 = p2solve(map, s2);

    // Solve the quadratic equation
    let c = p0;
    let f1_0 = p1 - p0;
    let f2_1 = p2 - p1;
    let a = (f2_1 - f1_0) / 2;
    let b = p1 - a - c;

    #[cfg(debug_assertions)]
    {
        println!("Equation 1: f(0) = 0a + 0b + c = {p0}");
        println!("Equation 2: f(1) = 1a + 1b + c = {p1}");
        println!("Equation 3: f(2) = 4a + 2b + c = {p2}");
        println!("f(1) - f(0) => a + b = {p1} - {p0} => a + b = {}", f1_0);
        println!("f(2) - f(1) => 3a + b = {p2} - {p1} => 3a + b = {}", f2_1);
        println!("(3a + b) - (a + b) = 2a = {f2_1} - {f1_0} => a = {a}");
        println!("a + b + c = {p1} => b = {p1} - a - c => b = {b}");

        println!("Equation: {a} x^2 + {b} x + {c}, with x = {div}");
    }

    (a * div * div + b * div + p0) as u64
}

fn find_start(map: &[InputEnt]) -> (usize, usize) {
    map.iter()
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

fn p2solve(map: &[InputEnt], steps: usize) -> i64 {
    let max_x = (map[0].len() - 1) as isize;
    let max_y = (map.len() - 1) as isize;

    let mut queue = VecDeque::new();
    let mut visited = HashMap::new();

    let (sx, sy) = find_start(map);

    queue.push_back((0, 0, sx as isize, sy as isize, 0));
    visited.insert((0, 0, sx as isize, sy as isize), 0);

    while let Some((sqx, sqy, x, y, cur_steps)) = queue.pop_front() {
        let mut move_to = |mut nx: isize, mut ny: isize| {
            let mut nsqx = sqx;
            let mut nsqy = sqy;

            if nx < 0 {
                nsqx -= 1;
                nx = max_x;
            }

            if nx > max_x {
                nsqx += 1;
                nx = 0;
            }

            if ny < 0 {
                nsqy -= 1;
                ny = max_y;
            }

            if ny > max_y {
                nsqy += 1;
                ny = 0;
            }

            if map[ny as usize][nx as usize] != Square::Rock
                && !visited.contains_key(&(nsqx, nsqy, nx, ny))
            {
                if cur_steps >= steps {
                    return;
                }

                queue.push_back((nsqx, nsqy, nx, ny, cur_steps + 1));
                visited.insert((nsqx, nsqy, nx, ny), cur_steps + 1);
            }
        };

        // North
        move_to(x, y - 1);

        // East
        move_to(x + 1, y);

        // South
        move_to(x, y + 1);

        // West
        move_to(x - 1, y);
    }

    visited
        .iter()
        .filter(|(_, s)| **s & 1 == steps & 0x01)
        .count() as i64
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
        assert_eq!(p2solve(&input, 6), 16);
        assert_eq!(p2solve(&input, 10), 50);
        assert_eq!(p2solve(&input, 50), 1594);
        assert_eq!(p2solve(&input, 100), 6536);
        assert_eq!(p2solve(&input, 500), 167004);
        assert_eq!(p2solve(&input, 1000), 668697);
        // assert_eq!(p2solve(&input, 5000), 16733044);
    }
}
