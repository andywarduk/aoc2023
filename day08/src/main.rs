use std::{collections::HashMap, error::Error};

use aoc::input::parse_input_vec;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(8, input_transform)?;
    let (dirs, loc_map) = parse_input(&input);

    // Run parts
    println!("Part 1: {}", part1(&dirs, &loc_map));
    println!("Part 2: {}", part2(&dirs, &loc_map));

    Ok(())
}

fn part1(dirs: &[Dir], loc_map: &HashMap<String, (String, String)>) -> u64 {
    let mut steps = 0;

    // Set start location
    let mut cur_loc = &String::from("AAA");

    // Walk through direction list
    for d in dirs.iter().cycle() {
        // Get map entry
        let ent = loc_map.get(cur_loc).unwrap();

        // Walk in direction
        cur_loc = match d {
            Dir::Left => &ent.0,
            Dir::Right => &ent.1,
        };

        steps += 1;

        // Finished?
        if cur_loc == "ZZZ" {
            break;
        }
    }

    steps
}

fn part2(dirs: &[Dir], loc_map: &HashMap<String, (String, String)>) -> u64 {
    // Find starting locations
    let start_locs = loc_map
        .iter()
        .filter_map(|(loc, _)| if loc.ends_with('A') { Some(loc) } else { None })
        .collect::<Vec<&String>>();

    // Calculate the repeat cycle for each start point
    let repeat_cycle = start_locs
        .iter()
        .map(|s| {
            let mut cur_loc = &s.to_string();
            let mut steps = 0;

            for d in dirs.iter().cycle() {
                let ent = loc_map.get(cur_loc).unwrap();

                cur_loc = match d {
                    Dir::Left => &ent.0,
                    Dir::Right => &ent.1,
                };

                steps += 1;

                if cur_loc.ends_with('Z') {
                    break;
                }
            }

            steps as u64
        })
        .collect::<Vec<u64>>();

    // Calculate the LCM of the repeat cycles
    repeat_cycle.into_iter().reduce(lcm).unwrap()
}

// From https://en.wikipedia.org/wiki/Least_common_multiple
fn lcm(l: u64, r: u64) -> u64 {
    (l * r) / gcd(l, r)
}

// From https://en.wikipedia.org/wiki/Binary_GCD_algorithm
pub fn gcd(mut u: u64, mut v: u64) -> u64 {
    let ored = u | v;

    if u == 0 || v == 0 {
        return ored;
    }

    // 'trailing_zeros' quickly counts a binary number's trailing zeros, giving its prime factorization's exponent on two
    let gcd_exponent_on_two = ored.trailing_zeros();

    // `>>=` divides the left by two to the power of the right, storing that in the left variable
    // `u` divided by its prime factorization's power of two turns it odd
    u >>= u.trailing_zeros();
    v >>= v.trailing_zeros();

    while u != v {
        if u < v {
            // Swap the variables' values with each other.
            core::mem::swap(&mut u, &mut v);
        }
        u -= v;
        u >>= u.trailing_zeros();
    }

    // `<<` multiplies the left by two to the power of the right
    u << gcd_exponent_on_two
}

enum Dir {
    Left,
    Right,
}

// Input parsing

fn input_transform(line: String) -> String {
    line
}

fn parse_input(lines: &[String]) -> (Vec<Dir>, HashMap<String, (String, String)>) {
    let mut line_iter = lines.iter();

    let dirs = line_iter
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            'L' => Dir::Left,
            'R' => Dir::Right,
            _ => panic!("Invalid direction"),
        })
        .collect();

    line_iter.next();

    let mut loc_map = HashMap::new();

    for line in line_iter {
        let mut split1 = line.split('=');
        let loc = split1.next().unwrap().trim().to_string();
        let dirs = split1
            .next()
            .unwrap()
            .trim()
            .trim_start_matches('(')
            .trim_end_matches(')');
        let mut split2 = dirs.split(',');
        let left = split2.next().unwrap().to_string();
        let right = split2.next().unwrap().trim().to_string();
        loc_map.insert(loc, (left, right));
    }

    (dirs, loc_map)
}

#[cfg(test)]
mod tests {
    use aoc::input::parse_test_vec;

    use super::*;

    const EXAMPLE1: &str = "\
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    const EXAMPLE2: &str = "\
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn test1() {
        let input = parse_test_vec(EXAMPLE1, input_transform).unwrap();
        let (dirs, loc_map) = parse_input(&input);

        assert_eq!(part1(&dirs, &loc_map), 6);
    }

    #[test]
    fn test2() {
        let input = parse_test_vec(EXAMPLE2, input_transform).unwrap();
        let (dirs, loc_map) = parse_input(&input);

        assert_eq!(part2(&dirs, &loc_map), 6);
    }
}
