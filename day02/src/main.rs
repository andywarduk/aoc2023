use std::{cmp::max, error::Error};

use aoc::input::parse_input_vec;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(2, input_transform)?;

    // Run parts
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

fn part1(input: &[InputEnt]) -> u64 {
    input.iter().fold(0, |acc, game| {
        if game
            .hands
            .iter()
            .any(|hand| hand.r > 12 || hand.g > 13 || hand.b > 14)
        {
            acc
        } else {
            acc + game.game as u64
        }
    })
}

fn part2(input: &[InputEnt]) -> u64 {
    input
        .iter()
        .map(|game| {
            let (r, g, b) = game.hands.iter().fold((0, 0, 0), |(r, g, b), hand| {
                (max(r, hand.r), max(g, hand.g), max(b, hand.b))
            });

            r as u64 * g as u64 * b as u64
        })
        .sum()
}

struct Game {
    game: u16,
    hands: Vec<Hand>,
}

#[derive(Default)]
struct Hand {
    r: u16,
    g: u16,
    b: u16,
}

// Input parsing

type InputEnt = Game;

fn input_transform(line: String) -> InputEnt {
    let mut split1 = line.split(": ");

    let game = split1.next().unwrap();

    let game_no = game.split(' ').nth(1).unwrap().parse::<u16>().unwrap();

    let mut game = Game {
        game: game_no,
        hands: Vec::new(),
    };

    let hands = split1.next().unwrap();

    for hand_str in hands.split("; ") {
        let mut hand = Hand::default();

        for cube in hand_str.split(", ") {
            let mut terms = cube.split(' ');

            let count = terms.next().unwrap().parse::<u16>().unwrap();

            match terms.next().unwrap() {
                "red" => hand.r = count,
                "green" => hand.g = count,
                "blue" => hand.b = count,
                _ => panic!("Invalid colour"),
            }
        }

        game.hands.push(hand);
    }

    game
}

#[cfg(test)]
mod tests {
    use aoc::input::parse_test_vec;

    use super::*;

    const EXAMPLE1: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test1() {
        let input = parse_test_vec(EXAMPLE1, input_transform).unwrap();
        assert_eq!(part1(&input), 8);
        assert_eq!(part2(&input), 2286);
    }
}
