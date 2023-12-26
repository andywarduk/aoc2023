use std::error::Error;

use aoc::input::parse_input_vec;

use z3::ast::Ast;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(24, input_transform)?;

    // Run parts
    println!(
        "Part 1: {}",
        part1(&input, 200_000_000_000_000, 400_000_000_000_000)
    );
    println!("Part 2: {}", part2(&input));

    Ok(())
}

fn part1(input: &[HailStone], lower: i64, upper: i64) -> u64 {
    let mut count = 0;

    for (i, h1) in input.iter().enumerate() {
        let m1 = h1.vy as f64 / h1.vx as f64;
        let c1 = h1.y as f64 - (m1 * h1.x as f64);

        #[cfg(debug_assertions)]
        {
            println!("Line {i}:");
            println!(
                " x = {}, y = {}, vx = {}, vy = {}",
                h1.x, h1.y, h1.vx, h1.vy
            );
            println!(" y = {m1} x + {c1}");
        }

        for h2 in input.iter().skip(i + 1) {
            let m2 = h2.vy as f64 / h2.vx as f64;
            let c2 = h2.y as f64 - (m2 * h2.x as f64);

            #[cfg(debug_assertions)]
            {
                println!(
                    "  x = {}, y = {}, vx = {}, vy = {}",
                    h2.x, h2.y, h2.vx, h2.vy
                );
                println!("  y = {m2} x + {c2}");
            }

            // y: m2 * x + c2 = m1 * x + c1
            // (m2-m1) x + c2 = c1
            // (m2-m1) x = c1 - c2
            let xi = (c1 - c2) / (m2 - m1);
            // Substitute x in line equation
            let yi = (m1 * xi) + c1;

            #[cfg(debug_assertions)]
            print!("    intercept : {xi} {yi}");
            if xi >= lower as f64 && xi <= upper as f64 && yi >= lower as f64 && yi <= upper as f64
            {
                if (xi - h1.x as f64).signum() == (h1.vx as f64).signum()
                    && (yi - h1.y as f64).signum() == (h1.vy as f64).signum()
                    && (xi - h2.x as f64).signum() == (h2.vx as f64).signum()
                    && (yi - h2.y as f64).signum() == (h2.vy as f64).signum()
                {
                    #[cfg(debug_assertions)]
                    println!(" (inside)");
                    count += 1;
                } else {
                    #[cfg(debug_assertions)]
                    println!(" (past)");
                }
            } else {
                #[cfg(debug_assertions)]
                println!(" (outside)");
            }
        }
    }

    count
}

fn part2(input: &[HailStone]) -> u64 {
    let cfg = z3::Config::new();
    let context = z3::Context::new(&cfg);
    let solver = z3::Solver::new(&context);

    let x = z3::ast::Int::new_const(&context, "x");
    let y = z3::ast::Int::new_const(&context, "y");
    let z = z3::ast::Int::new_const(&context, "z");

    let vx = z3::ast::Int::new_const(&context, "vx");
    let vy = z3::ast::Int::new_const(&context, "vy");
    let vz = z3::ast::Int::new_const(&context, "vz");

    for (i, hs) in input.iter().take(3).enumerate() {
        let a = z3::ast::Int::from_i64(&context, hs.x);
        let b = z3::ast::Int::from_i64(&context, hs.y);
        let c = z3::ast::Int::from_i64(&context, hs.z);

        let va = z3::ast::Int::from_i64(&context, hs.vx);
        let vb = z3::ast::Int::from_i64(&context, hs.vy);
        let vc = z3::ast::Int::from_i64(&context, hs.vz);

        let t = z3::ast::Int::new_const(&context, format!("t{i}"));

        solver.assert(&t.gt(&z3::ast::Int::from_i64(&context, 0)));
        solver.assert(&(x.clone() + vx.clone() * t.clone())._eq(&(a + va * t.clone())));
        solver.assert(&(y.clone() + vy.clone() * t.clone())._eq(&(b + vb * t.clone())));
        solver.assert(&(z.clone() + vz.clone() * t.clone())._eq(&(c + vc * t.clone())));
    }

    let result = if solver.check() == z3::SatResult::Sat {
        let Some(m) = solver.get_model() else {
            panic!("Failed to solve!");
        };
        m.eval(&(x + y + z), true).unwrap()
    } else {
        panic!("Not satisfied")
    };

    result.as_u64().unwrap()
}

// Input parsing

struct HailStone {
    x: i64,
    y: i64,
    z: i64,
    vx: i64,
    vy: i64,
    vz: i64,
}

fn input_transform(line: String) -> HailStone {
    let mut split = line.split(" @ ");

    let pos = split
        .next()
        .unwrap()
        .split(", ")
        .map(|s| s.trim().parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let vel = split
        .next()
        .unwrap()
        .split(", ")
        .map(|s| s.trim().parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    HailStone {
        x: pos[0],
        y: pos[1],
        z: pos[2],
        vx: vel[0],
        vy: vel[1],
        vz: vel[2],
    }
}

#[cfg(test)]
mod tests {
    use aoc::input::parse_test_vec;

    use super::*;

    const EXAMPLE1: &str = "\
19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";

    #[test]
    fn test1() {
        let input = parse_test_vec(EXAMPLE1, input_transform).unwrap();
        assert_eq!(part1(&input, 7, 27), 2);
        assert_eq!(part2(&input), 47);
    }
}
