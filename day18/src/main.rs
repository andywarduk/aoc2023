use std::{
    cmp::{max, min},
    error::Error,
};

use aoc::input::parse_input_vec;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(18, input_transform1)?;

    // Run part 1
    println!("Part 1: {}", calc_area(&input));

    // Get input
    let input = parse_input_vec(18, input_transform2)?;

    // Run part 2
    println!("Part 2: {}", calc_area(&input));

    Ok(())
}

/// Plan line
#[derive(Debug)]
struct Line {
    sx: i64,
    sy: i64,
    ex: i64,
    ey: i64,
}

impl Line {
    /// Returns the direction of the plan line
    fn direction(&self) -> Dir {
        if self.sx == self.ex {
            // Horizontal
            if self.sy < self.ey {
                Dir::Down
            } else {
                Dir::Up
            }
        } else {
            // Vertical
            if self.sx < self.ex {
                Dir::Right
            } else {
                Dir::Left
            }
        }
    }
}

/// Line direction
#[derive(PartialEq)]
enum Dir {
    Up = 1,
    Down = 2,
    Left = 4,
    Right = 8,
}

/// Convert plan to lines
fn get_lines(plan: &[PlanStep]) -> Vec<Line> {
    let mut lines = Vec::new();

    let mut x = 0i64;
    let mut y = 0i64;

    // Convert plan to lines
    for step in plan {
        let mut line = Line {
            sx: x,
            sy: y,
            ex: 0,
            ey: 0,
        };

        match step.dir {
            Dir::Up => y -= step.amount as i64,
            Dir::Down => y += step.amount as i64,
            Dir::Left => x -= step.amount as i64,
            Dir::Right => x += step.amount as i64,
        }

        line.ex = x;
        line.ey = y;

        lines.push(line)
    }

    lines
}

/// Calculate the area enclosed by the trench
fn calc_area(plan: &[PlanStep]) -> i64 {
    let lines = get_lines(plan);

    // Get interesting y coordinates
    let mut ys = lines
        .iter()
        .flat_map(|l| {
            if l.sy == l.ey {
                vec![l.sy]
            } else {
                vec![l.sy, l.ey]
            }
        })
        .collect::<Vec<_>>();

    ys.sort();
    ys.dedup();

    let mut total_area = 0;
    let mut last_y = i64::MIN;
    let mut last_area = 0;

    for y in ys.iter() {
        if last_area != 0 {
            total_area += (*y - last_y) * last_area;
        }

        let area = calc_line_area(*y, &lines);
        total_area += area;

        last_y = *y + 1;
        last_area = calc_line_area(last_y, &lines);
    }

    total_area
}

/// Calculate area for this y
fn calc_line_area(y: i64, lines: &[Line]) -> i64 {
    // Find vertical lines that intersect this y
    let mut yvlines = lines
        .iter()
        .filter(|line| {
            line.sy != line.ey && min(line.sy, line.ey) <= y && max(line.sy, line.ey) >= y
        })
        .collect::<Vec<_>>();

    // Sort by x position
    yvlines.sort_by(|a, b| a.sx.cmp(&b.sx));

    // Called when a trench is being crossed
    let cross = |count: &mut usize, in_dir: &mut Option<Dir>| {
        *count += 1;
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
                cross(count, in_dir)
            }
        }
        None => {
            // In in a direction
            *in_dir = Some(dir)
        }
    };

    let mut crossings = 0;
    let mut in_dir = None;
    let mut in_x = None;
    let mut area = 0;

    for vline in yvlines {
        let x = vline.sx;

        if vline.sy == y || vline.ey == y {
            // Line ends or starts on this y
            in_out(vline.direction(), &mut crossings, &mut in_dir);
        } else {
            // Crossing the line
            cross(&mut crossings, &mut in_dir);
        }

        if crossings & 0x01 == 0x01 || in_dir.is_some() {
            // Inside
            if in_x.is_none() {
                in_x = Some(x)
            }
        } else {
            // Outside
            if let Some(last_x) = in_x {
                area += (x - last_x) + 1;
                in_x = None;
            }
        }
    }

    area
}

// Input parsing

struct PlanStep {
    dir: Dir,
    amount: u64,
}

fn input_transform1(line: String) -> PlanStep {
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

fn input_transform2(line: String) -> PlanStep {
    let code = line.split('#').nth(1).unwrap().trim_end_matches(')');

    let amount = u64::from_str_radix(&code[0..5], 16).unwrap();
    let dir = match &code[5..6] {
        "3" => Dir::Up,
        "1" => Dir::Down,
        "2" => Dir::Left,
        "0" => Dir::Right,
        _ => panic!("Bad direction"),
    };

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
        let input = parse_test_vec(EXAMPLE1, input_transform1).unwrap();
        assert_eq!(calc_area(&input), 62);
    }

    #[test]
    fn test2() {
        let input = parse_test_vec(EXAMPLE1, input_transform2).unwrap();
        assert_eq!(calc_area(&input), 952408144115);
    }
}
