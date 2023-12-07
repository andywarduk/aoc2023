use std::error::Error;

use aoc::input::parse_input_vec;

use crate::hands::{HandP1, HandP2, HandStrength, CARDSP1, CARDSP2};

mod hands;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(7, input_transform)?;

    // Run parts
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

fn part1(input: &[InputEnt]) -> u64 {
    // Map input to part 1 hand
    let mut input = input
        .iter()
        .map(|h| HandP1 {
            cards: h.cards.clone(),
            hand_type: HandStrength::from_cards(&h.cards, &CARDSP1),
            bid: h.bid,
        })
        .collect::<Vec<HandP1>>();

    // Sort by strength
    input.sort();

    #[cfg(test)]
    println!("{:#?}", input);

    // Calculate winnings
    input
        .iter()
        .enumerate()
        .map(|(i, h)| (i as u64 + 1) * h.bid)
        .sum()
}

fn part2(input: &[InputEnt]) -> u64 {
    // Map input to part 2 hand
    let mut input = input
        .iter()
        .map(|h| HandP2 {
            cards: h.cards.clone(),
            hand_type: HandStrength::from_cards_with_jokers(&h.cards, &CARDSP2),
            bid: h.bid,
        })
        .collect::<Vec<HandP2>>();

    // Sort by strength
    input.sort();

    #[cfg(test)]
    println!("{:#?}", input);

    // Calculate winnings
    input
        .iter()
        .enumerate()
        .map(|(i, h)| (i as u64 + 1) * h.bid)
        .sum()
}

// Input parsing

#[derive(Debug, Clone)]
struct InputEnt {
    cards: Vec<char>,
    bid: u64,
}

fn input_transform(line: String) -> InputEnt {
    let mut terms = line.split_ascii_whitespace();

    InputEnt {
        cards: terms.next().unwrap().chars().collect::<Vec<char>>(),
        bid: terms.next().unwrap().parse::<u64>().unwrap(),
    }
}

#[cfg(test)]
mod tests {
    use aoc::input::parse_test_vec;

    use super::*;

    const EXAMPLE1: &str = "\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn test1() {
        let input = parse_test_vec(EXAMPLE1, input_transform).unwrap();
        assert_eq!(part1(&input), 6440);
        assert_eq!(part2(&input), 5905);
    }
}
