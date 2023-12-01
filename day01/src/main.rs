use std::error::Error;

use aoc::input::parse_input_vec;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(1, input_transform)?;

    // Run parts
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

fn part1(input: &[InputEnt]) -> u64 {
    input.iter().map(|(i, _)| *i as u64).sum()
}

fn part2(input: &[InputEnt]) -> u64 {
    input.iter().map(|(_, i)| *i as u64).sum()
}

// Input parsing

type InputEnt = (u8, u8);

const NUMSTR: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn input_transform(line: String) -> InputEnt {
    // Part 1
    let mut iter = line.chars().filter(|c| c.is_numeric());
    let first = iter.next().unwrap_or('0');
    let last = iter.last().unwrap_or(first);
    let p1 = ((first as u8 - b'0') * 10) + (last as u8 - b'0');

    // Part 2
    let mut iter = (0..line.len()).filter_map(|p| {
        let sub = &line[p..];
        let c = sub.as_bytes()[0] as char;

        if c.is_numeric() {
            Some(c as u8 - b'0')
        } else {
            NUMSTR
                .iter()
                .enumerate()
                .filter_map(|(i, n)| {
                    if sub.starts_with(n) {
                        Some(i as u8)
                    } else {
                        None
                    }
                })
                .next()
        }
    });
    let first = iter.next().unwrap_or(0);
    let last = iter.last().unwrap_or(first);
    let p2 = (first * 10) + last;

    (p1, p2)
}

#[cfg(test)]
mod tests {
    use aoc::input::parse_test_vec;

    use super::*;

    const EXAMPLE1: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    const EXAMPLE2: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    fn test1() {
        let input = parse_test_vec(EXAMPLE1, input_transform).unwrap();
        assert_eq!(part1(&input), 142);

        let input = parse_test_vec(EXAMPLE2, input_transform).unwrap();
        assert_eq!(part2(&input), 281);
    }
}
