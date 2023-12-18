use std::{
    cmp::{max, min},
    error::Error,
};

use aoc::input::parse_input_vec;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(18, input_transform)?;

    // Run parts
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

fn part1(plan: &[PlanStep]) -> u64 {
    let mut min_x = i64::MAX;
    let mut max_x = 0i64;
    let mut min_y = i64::MAX;
    let mut max_y = 0i64;

    let mut x = 0i64;
    let mut y = 0i64;

    for step in plan {
        match step.dir {
            Dir::Up => y -= step.amount as i64,
            Dir::Down => y += step.amount as i64,
            Dir::Left => x -= step.amount as i64,
            Dir::Right => x += step.amount as i64,
        }

        min_x = min(x, min_x);
        max_x = max(x, max_x);
        min_y = min(y, min_y);
        max_y = max(y, max_y);
    }

    let map_row = vec![[None; 2]; (max_x - min_x) as usize + 1];
    let mut map = vec![map_row.clone(); (max_y - min_y) as usize + 1];

    let mut x = -min_x as usize;
    let mut y = -min_y as usize;

    for step in plan {
        let iter: Box<dyn Iterator<Item = (usize, usize)>> = match step.dir {
            Dir::Up => Box::new((0..=(step.amount as usize)).map(|dy| (x, y - dy))),
            Dir::Down => Box::new((0..=(step.amount as usize)).map(|dy| (x, y + dy))),
            Dir::Left => Box::new((0..=(step.amount as usize)).map(|dx| (x - dx, y))),
            Dir::Right => Box::new((0..=(step.amount as usize)).map(|dx| (x + dx, y))),
        };

        for (x, y) in iter {
            match step.dir {
                Dir::Up | Dir::Down => map[y][x][0] = Some(step.dir),
                _ => map[y][x][1] = Some(step.dir),
            }
        }

        match step.dir {
            Dir::Up => y -= step.amount as usize,
            Dir::Down => y += step.amount as usize,
            Dir::Left => x -= step.amount as usize,
            Dir::Right => x += step.amount as usize,
        }
    }

    // Called when a pipe is being crossed
    let cross_pipe = |pipe_count: &mut usize, in_dir: &mut Option<Dir>| {
        *pipe_count += 1;
        *in_dir = None
    };

    // Called when coming in in a given direction
    let in_out = |dir: Dir, count: &mut usize, in_dir: &mut Option<Dir>| match in_dir {
        Some(cur_dir) => {
            if *cur_dir != dir {
                // In and out same direction
                *in_dir = None
            } else {
                // In and out in opposite directions
                cross_pipe(count, in_dir)
            }
        }
        None => {
            // In in a direction
            *in_dir = Some(dir)
        }
    };

    let map = map
        .iter()
        .map(|line| {
            let mut count = 0;
            let mut in_dir = None;

            line.iter()
                .map(|cell| match cell[0] {
                    Some(Dir::Up) | Some(Dir::Down) => {
                        if cell[1].is_none() {
                            cross_pipe(&mut count, &mut in_dir);
                            'X'
                        } else {
                            in_out(cell[0].unwrap(), &mut count, &mut in_dir);
                            'x'
                        }
                    }
                    None => match cell[1] {
                        Some(_) => 'Y',
                        None => {
                            if count % 2 == 1 {
                                '#'
                            } else {
                                '.'
                            }
                        }
                    },
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    map.iter()
        .map(|line| line.iter().filter(|c| **c != '.').count())
        .sum::<usize>() as u64
}

fn part2(input: &[PlanStep]) -> u64 {
    0 // TODO
}

// Input parsing

struct PlanStep {
    dir: Dir,
    amount: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dir {
    Up = 1,
    Down = 2,
    Left = 4,
    Right = 8,
}

enum Trench {
    Empty,
    NS,
    EW,
    NE,
    NW,
    SE,
    SW,
}

fn input_transform(line: String) -> PlanStep {
    let mut split = line.split_ascii_whitespace();

    let dir = match split.next().unwrap() {
        "U" => Dir::Up,
        "D" => Dir::Down,
        "L" => Dir::Left,
        "R" => Dir::Right,
        _ => panic!("Bad direction"),
    };

    let amount = split.next().unwrap().parse::<u64>().unwrap();

    PlanStep { dir, amount }
}

#[cfg(test)]
mod tests {
    use aoc::input::parse_test_vec;

    use super::*;

    const EXAMPLE1: &str = "\
R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

    #[test]
    fn test1() {
        let input = parse_test_vec(EXAMPLE1, input_transform).unwrap();
        assert_eq!(part1(&input), 62);
        assert_eq!(part2(&input), 0 /* TODO */);
    }
}
