use std::error::Error;

use aoc::input::parse_input_vec;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(9, input_transform)?;

    // Run parts
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

fn part1(input: &[InputEnt]) -> i64 {
    let mut result: i64 = 0;

    for v in input {
        result += v[v.len() - 1] + calc_diffs_right(v);
    }

    result
}

fn calc_diffs_right(v1: &[i64]) -> i64 {
    let v2 = v1.windows(2).map(|x| x[1] - x[0]).collect::<Vec<i64>>();

    if v2.iter().any(|v| *v != 0) {
        v2[v2.len() - 1] + calc_diffs_right(&v2)
    } else {
        0
    }
}

fn part2(input: &[InputEnt]) -> i64 {
    let mut result: i64 = 0;

    for v in input {
        result += v[0] - calc_diffs_left(v);
    }

    result
}

fn calc_diffs_left(v1: &[i64]) -> i64 {
    let v2 = v1.windows(2).map(|x| x[1] - x[0]).collect::<Vec<i64>>();

    if v2.iter().any(|v| *v != 0) {
        v2[0] - calc_diffs_left(&v2)
    } else {
        0
    }
}

// Input parsing

type InputEnt = Vec<i64>;

fn input_transform(line: String) -> InputEnt {
    line.split_ascii_whitespace()
        .map(|t| t.parse::<i64>().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use aoc::input::parse_test_vec;

    use super::*;

    const EXAMPLE1: &str = "\
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn test1() {
        let input = parse_test_vec(EXAMPLE1, input_transform).unwrap();
        assert_eq!(part1(&input), 114);
        assert_eq!(part2(&input), 2);
    }
}
