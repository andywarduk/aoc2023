use std::error::Error;

use aoc::input::parse_input_vec;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec($day, input_transform)?;

    // Run parts
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

fn part1(input: &[InputEnt]) -> u64 {
    0 // TODO
}

fn part2(input: &[InputEnt]) -> u64 {
    0 // TODO
}

// Input parsing

type InputEnt = String; // TODO

fn input_transform(line: String) -> InputEnt {
    // TODO
    line
}

#[cfg(test)]
mod tests {
    use aoc::input::parse_test_vec;

    use super::*;

    const EXAMPLE1: &str = "TODO";

    #[test]
    fn test1() {
        let input = parse_test_vec(EXAMPLE1, input_transform).unwrap();
        assert_eq!(part1(&input), 0 /* TODO */);
        assert_eq!(part2(&input), 0 /* TODO */);
    }
}
