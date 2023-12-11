use std::error::Error;

use aoc::input::parse_input_vec;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(11, input_transform)?;

    // Run parts
    println!("Part 1: {}", dists_with_expansion(&input, 2));
    println!("Part 2: {}", dists_with_expansion(&input, 1_000_000));

    Ok(())
}

fn dists_with_expansion(input: &[InputEnt], expansion: u64) -> u64 {
    // Find empty rows and columns
    let mut x_found = vec![false; input[0].len()];

    input.iter().for_each(|l| {
        l.iter()
            .enumerate()
            .filter(|(_, g)| **g)
            .for_each(|(x, _)| x_found[x] = true)
    });

    let y_found = input
        .iter()
        .map(|l| l.iter().any(|g| *g))
        .collect::<Vec<bool>>();

    // Create x and y mapping
    let mut xmap = Vec::new();
    let mut ymap = Vec::new();

    let mut mapped = 0;
    for found in x_found.into_iter() {
        mapped += if found { 1u64 } else { expansion };
        xmap.push(mapped);
    }

    let mut mapped = 0;
    for found in y_found.into_iter() {
        mapped += if found { 1u64 } else { expansion };
        ymap.push(mapped);
    }

    // Get galaxy positions via mappings
    let positions = input
        .iter()
        .enumerate()
        .fold(Vec::new(), |mut positions, (y, l)| {
            l.iter()
                .enumerate()
                .fold(&mut positions, |positions, (x, g)| {
                    if *g {
                        positions.push((xmap[x], ymap[y]));
                    }

                    positions
                });

            positions
        });

    // Find distances
    let mut dist_sum = 0;

    for (i, (x1, y1)) in positions.iter().enumerate() {
        for (x2, y2) in positions[i + 1..].iter() {
            let dist = i64::abs(*x1 as i64 - *x2 as i64) + i64::abs(*y1 as i64 - *y2 as i64);
            dist_sum += dist as u64;
        }
    }

    dist_sum
}

// Input parsing

type InputEnt = Vec<bool>;

fn input_transform(line: String) -> InputEnt {
    line.chars().map(|c| c == '#').collect::<Vec<bool>>()
}

#[cfg(test)]
mod tests {
    use aoc::input::parse_test_vec;

    use super::*;

    const EXAMPLE1: &str = "\
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn test1() {
        let input = parse_test_vec(EXAMPLE1, input_transform).unwrap();
        assert_eq!(dists_with_expansion(&input, 2), 374);
        assert_eq!(dists_with_expansion(&input, 10), 1030);
        assert_eq!(dists_with_expansion(&input, 100), 8410);
    }
}
