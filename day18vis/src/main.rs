use std::{
    cmp::{max, min},
    error::Error,
    fs::File,
    io::Write,
};

use aoc::input::parse_input_vec;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(18, input_transform1)?;

    // Run part 1
    draw_trench(&input, "vis/day18-1.svg")?;

    // Get input
    let input = parse_input_vec(18, input_transform2)?;

    // Run part 2
    draw_trench(&input, "vis/day18-2.svg")?;

    Ok(())
}

fn draw_trench(plan: &[PlanStep], file: &str) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(file)?;

    let mut x = 0i64;
    let mut y = 0i64;

    let mut min_x = i64::MAX;
    let mut min_y = i64::MAX;
    let mut max_x = i64::MIN;
    let mut max_y = i64::MIN;

    for e in plan {
        match e.dir {
            PlanDir::Up => y -= e.amount as i64,
            PlanDir::Down => y += e.amount as i64,
            PlanDir::Left => x -= e.amount as i64,
            PlanDir::Right => x += e.amount as i64,
        }

        min_x = min(min_x, x);
        min_y = min(min_y, y);
        max_x = max(max_x, x);
        max_y = max(max_y, y);
    }

    let y_height = max_y - min_y;
    let x_width = max_x - min_x;

    let height = 900;
    let ratio = height as f64 / y_height as f64;
    let width = (x_width as f64 * ratio) as i32;

    file.write_fmt(format_args!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"no\"?>\n"
    ))?;
    file.write_fmt(format_args!("<!DOCTYPE svg PUBLIC \"-//W3C//DTD SVG 1.1//EN\" \"http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd\">\n"))?;

    file.write_fmt(format_args!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{width}\" height=\"{height}\" viewBox=\"{min_x} {min_y} {x_width} {y_height}\">\n"
    ))?;

    file.write_fmt(format_args!("<path d=\"M 0 0"))?;

    let mut x = 0i64;
    let mut y = 0i64;

    for e in plan {
        match e.dir {
            PlanDir::Up => y -= e.amount as i64,
            PlanDir::Down => y += e.amount as i64,
            PlanDir::Left => x -= e.amount as i64,
            PlanDir::Right => x += e.amount as i64,
        }
        file.write_fmt(format_args!(" L {x} {y}"))?;
    }

    file.write_fmt(format_args!(
        "Z \" stroke=\"black\" stroke-width=\"0.15%\" fill=\"red\"/>"
    ))?;

    file.write_fmt(format_args!("</svg>\n"))?;

    Ok(())
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
