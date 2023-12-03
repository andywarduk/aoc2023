use std::{cmp::min, collections::HashMap, error::Error};

use aoc::input::parse_input_vec;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(3, input_transform)?;

    // Run parts
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

fn part1(input: &[InputEnt]) -> u64 {
    let mut result = 0;

    let mut process_number = |y: usize, x1: usize, x2: usize| {
        // Iterate surrounding lines
        'outer: for line in input
            .iter()
            .take(min(input.len(), y + 2))
            .skip(y.saturating_sub(1))
        {
            // Iterate surrounding characters
            for c in line
                .iter()
                .take(min(line.len(), x2 + 2))
                .skip(x1.saturating_sub(1))
            {
                // Look for symbol (not . or numeric)
                if *c != '.' && !(*c).is_numeric() {
                    // Found symbol - add part number and break
                    result += input[y][x1..=x2]
                        .iter()
                        .collect::<String>()
                        .parse::<u64>()
                        .unwrap();

                    break 'outer;
                }
            }
        }
    };

    // Iterate input lines
    for (y, line) in input.iter().enumerate() {
        let mut number_start = None;

        // Iterate characters in the line
        for (x, c) in line.iter().enumerate() {
            // In a number?
            if let Some(start) = number_start {
                // Yes - is this character non-numeric?
                if !c.is_numeric() {
                    // Yes - process the number
                    process_number(y, start, x - 1);
                    number_start = None;
                }
            } else if c.is_numeric() {
                // Not in a number but numeric character found - start the number
                number_start = Some(x);
            }
        }

        if let Some(start) = number_start {
            // Process number at end of line
            process_number(y, start, line.len() - 1)
        }
    }

    result
}

fn part2(input: &[InputEnt]) -> u64 {
    let mut result = 0;

    // Map of gear positions to adjacent part numbers
    let mut gears: HashMap<(usize, usize), Vec<u64>> = HashMap::new();

    let mut process_number = |y: usize, x1: usize, x2: usize| {
        // Get part number
        let part_no = input[y][x1..=x2]
            .iter()
            .collect::<String>()
            .parse::<u64>()
            .unwrap();

        // Iterate surrounding lines
        for (y, line) in input
            .iter()
            .enumerate()
            .take(min(input.len(), y + 2))
            .skip(y.saturating_sub(1))
        {
            // Iterate surrounding characters
            for (x, c) in line
                .iter()
                .enumerate()
                .take(min(line.len(), x2 + 2))
                .skip(x1.saturating_sub(1))
            {
                // Got a gear?
                if *c == '*' {
                    // Yes - add part number to this gear
                    gears.entry((x, y)).or_default().push(part_no);
                }
            }
        }
    };

    // Iterate input lines
    for (y, line) in input.iter().enumerate() {
        let mut number_start = None;

        // Iterate characters in the line
        for (x, c) in line.iter().enumerate() {
            // In a number?
            if let Some(start) = number_start {
                // Yes - is this character non-numeric?
                if !c.is_numeric() {
                    // Yes - process the number
                    process_number(y, start, x - 1);
                    number_start = None;
                }
            } else if c.is_numeric() {
                // Not in a number but numeric character found - start the number
                number_start = Some(x);
            }
        }

        if let Some(start) = number_start {
            // Process number at end of line
            process_number(y, start, line.len() - 1)
        }
    }

    // Iterate found gears
    for (_, adjacent) in gears {
        // Exactly two adjacent part numbers?
        if adjacent.len() == 2 {
            // Add the gear ratio
            result += adjacent[0] * adjacent[1]
        }
    }

    result
}

// Input parsing

type InputEnt = Vec<char>;

fn input_transform(line: String) -> InputEnt {
    line.chars().collect()
}

#[cfg(test)]
mod tests {
    use aoc::input::parse_test_vec;

    use super::*;

    const EXAMPLE1: &str = "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn test1() {
        let input = parse_test_vec(EXAMPLE1, input_transform).unwrap();
        assert_eq!(part1(&input), 4361);
        assert_eq!(part2(&input), 467835);
    }
}
