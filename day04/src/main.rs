use std::{collections::VecDeque, error::Error};

use aoc::input::parse_input_vec;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(4, input_transform)?;

    // Run parts
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

fn part1(input: &[Card]) -> u64 {
    input
        .iter()
        .map(|card| {
            // Calclate score
            let mut score = 0;

            for n in &card.winning {
                if card.actual.iter().any(|a| *a == *n) {
                    if score == 0 {
                        score = 1
                    } else {
                        score *= 2;
                    }
                }
            }

            score
        })
        .sum()
}

fn part2(input: &[Card]) -> u64 {
    let mut cards = 0;
    let mut queue = VecDeque::new();

    // Pre-calculate number of wins on each card
    let wins = input
        .iter()
        .map(|c| {
            c.winning
                .iter()
                .filter(|w| c.actual.iter().any(|a| *a == **w))
                .count()
        })
        .collect::<Vec<usize>>();

    // Fill the queue with each card
    for i in 0..input.len() {
        queue.push_back(i);
    }

    // Process the queue
    while let Some(ent) = queue.pop_front() {
        cards += 1;

        // Add new cards
        for i in (0..wins[ent]).rev() {
            queue.push_front(ent + i + 1)
        }
    }

    cards
}

// Input parsing

struct Card {
    winning: Vec<u8>,
    actual: Vec<u8>,
}

fn input_transform(line: String) -> Card {
    let numbers = line.split(':').nth(1).unwrap();

    let mut numsets = numbers.split('|');

    let winning = numsets
        .next()
        .unwrap()
        .split_whitespace()
        .map(|n| n.parse::<u8>().unwrap())
        .collect();

    let actual = numsets
        .next()
        .unwrap()
        .split_whitespace()
        .map(|n| n.parse::<u8>().unwrap())
        .collect();

    Card { winning, actual }
}

#[cfg(test)]
mod tests {
    use aoc::input::parse_test_vec;

    use super::*;

    const EXAMPLE1: &str = "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test1() {
        let input = parse_test_vec(EXAMPLE1, input_transform).unwrap();
        assert_eq!(part1(&input), 13);
        assert_eq!(part2(&input), 30);
    }
}
