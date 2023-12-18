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

/// Plan vertical line
#[derive(Debug)]
struct VLine {
    x: i64,
    sy: i64,
    ey: i64,
}

#[derive(PartialEq)]
enum VLineDir {
    Up,
    Down,
}

impl VLine {
    /// Returns the direction of the plan line
    fn direction(&self) -> VLineDir {
        if self.sy < self.ey {
            VLineDir::Down
        } else {
            VLineDir::Up
        }
    }
}

/// Convert plan to veritcal lines
fn get_vlines(plan: &[PlanStep]) -> Vec<VLine> {
    let mut lines = Vec::new();

    let mut x = 0i64;
    let mut y = 0i64;

    // Convert plan to lines
    for step in plan {
        let sy = y;

        match step.dir {
            PlanDir::Up => y -= step.amount as i64,
            PlanDir::Down => y += step.amount as i64,
            PlanDir::Left => x -= step.amount as i64,
            PlanDir::Right => x += step.amount as i64,
        }

        if sy != y {
            lines.push(VLine { x, sy, ey: y })
        }
    }

    lines
}

/// Calculate the area enclosed by the trench
fn calc_area(plan: &[PlanStep]) -> i64 {
    // Get vertical lines
    let lines = get_vlines(plan);

    // Get interesting y coordinates
    let mut ys = lines.iter().flat_map(|l| [l.sy, l.ey]).collect::<Vec<_>>();

    // Sort and deduplicate
    ys.sort();
    ys.dedup();

    // Calculate total area
    let (total_area, _, _) = ys.iter().fold(
        (0, i64::MIN, 0),
        |(mut total_area, mut last_y, mut last_area), &y| {
            // Add last lines
            if last_area != 0 {
                total_area += (y - last_y) * last_area;
            }

            // Add this line to total area
            total_area += calc_line_area(y, &lines);

            // Work out area of the next line and save
            last_y = y + 1;
            last_area = calc_line_area(last_y, &lines);

            (total_area, last_y, last_area)
        },
    );

    total_area
}

/// Calculate area for this y
fn calc_line_area(y: i64, lines: &[VLine]) -> i64 {
    // Find lines that intersect this y
    let mut yvlines = lines
        .iter()
        .filter(|line| min(line.sy, line.ey) <= y && max(line.sy, line.ey) >= y)
        .collect::<Vec<_>>();

    // Sort by x position
    yvlines.sort_by(|a, b| a.x.cmp(&b.x));

    // Calculate lagoon area
    let (area, _, _, _) = yvlines.iter().fold(
        (0, 0, None, None),
        |(mut area, mut crossings, mut in_dir, mut in_x), vline| {
            if vline.sy == y || vline.ey == y {
                // Line ends or starts on this y
                let dir = vline.direction();

                match in_dir {
                    Some(cur_dir) => {
                        if cur_dir == dir {
                            // In and out in different directions
                            crossings += 1;
                        }
                        in_dir = None;
                    }
                    None => {
                        // In in a direction
                        in_dir = Some(dir);
                    }
                }
            } else {
                // Crossing the line
                crossings += 1;
                in_dir = None;
            }

            if crossings & 0x01 == 0x01 || in_dir.is_some() {
                // Inside
                if in_x.is_none() {
                    in_x = Some(vline.x)
                }
            } else {
                // Outside
                if let Some(last_x) = in_x {
                    area += (vline.x - last_x) + 1;
                    in_x = None;
                }
            }

            (area, crossings, in_dir, in_x)
        },
    );

    area
}

// Input parsing

/// Plan step
struct PlanStep {
    dir: PlanDir,
    amount: u64,
}

/// Dig direction
#[derive(PartialEq)]
enum PlanDir {
    Up,
    Down,
    Left,
    Right,
}

/// Transform for part 1
fn input_transform1(line: String) -> PlanStep {
    let mut split = line.split_ascii_whitespace();

    let dir = match split.next().unwrap() {
        "U" => PlanDir::Up,
        "D" => PlanDir::Down,
        "L" => PlanDir::Left,
        "R" => PlanDir::Right,
        _ => panic!("Bad direction"),
    };

    let amount = split.next().unwrap().parse::<u64>().unwrap();

    PlanStep { dir, amount }
}

/// Transform for part 2
fn input_transform2(line: String) -> PlanStep {
    let code = line.split('#').nth(1).unwrap().trim_end_matches(')');

    let amount = u64::from_str_radix(&code[0..5], 16).unwrap();
    let dir = match &code[5..6] {
        "3" => PlanDir::Up,
        "1" => PlanDir::Down,
        "2" => PlanDir::Left,
        "0" => PlanDir::Right,
        _ => panic!("Bad direction"),
    };

    PlanStep { dir, amount }
}

// Tests

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
