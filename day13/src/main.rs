use std::{cmp::min, error::Error};

use aoc::input::parse_input_vec;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(13, input_transform)?;
    let input = build_boards(&input);

    // Run parts
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

fn part1(input: &[Board]) -> u64 {
    sum_reflections(input, 0)
}

fn part2(input: &[Board]) -> u64 {
    sum_reflections(input, 1)
}

#[derive(Debug)]
enum Reflection {
    Horiz(usize),
    Vert(usize),
}

fn sum_reflections(input: &[Board], dist: u64) -> u64 {
    input.iter().fold(0, |acc, board| {
        acc + match find_reflection(board, dist) {
            Reflection::Horiz(y) => y as u64 * 100,
            Reflection::Vert(x) => x as u64,
        }
    })
}

fn find_reflection(board: &Board, dist: u64) -> Reflection {
    // Horizontal
    for y in 1..board.len() {
        let reflection = Reflection::Horiz(y);

        if test_reflection(board, &reflection) == dist {
            return reflection;
        }
    }

    // Vertical
    for x in 1..board[0].len() {
        let reflection = Reflection::Vert(x);

        if test_reflection(board, &reflection) == dist {
            return reflection;
        }
    }

    panic!("No reflection")
}

fn cols_dist(board: &Board, c1: usize, c2: usize) -> u64 {
    board.iter().fold(
        0,
        |acc, line| if line[c1] != line[c2] { acc + 1 } else { acc },
    )
}

fn rows_dist(board: &Board, r1: usize, r2: usize) -> u64 {
    board[r1]
        .iter()
        .zip(&board[r2])
        .fold(0, |acc, (&p1, &p2)| if p1 != p2 { acc + 1 } else { acc })
}

fn test_reflection(board: &Board, reflection: &Reflection) -> u64 {
    match reflection {
        Reflection::Horiz(y) => (0..=min(y - 1, board.len() - (y + 1)))
            .map(|i| rows_dist(board, y - (i + 1), y + i))
            .sum(),
        Reflection::Vert(x) => (0..=min(x - 1, board[0].len() - (x + 1)))
            .map(|i| cols_dist(board, x - (i + 1), x + i))
            .sum(),
    }
}

// Input parsing

fn input_transform(line: String) -> String {
    line
}

type Board = Vec<Vec<bool>>;

fn build_boards(input: &[String]) -> Vec<Board> {
    let mut boards = Vec::new();
    let mut board = Vec::new();

    for line in input {
        if line.is_empty() {
            if !board.is_empty() {
                boards.push(board);
            }

            board = Vec::new();
        } else {
            board.push(
                line.chars()
                    .map(|c| match c {
                        '#' => true,
                        '.' => false,
                        _ => panic!("Invalid char"),
                    })
                    .collect::<Vec<_>>(),
            )
        }
    }

    if !board.is_empty() {
        boards.push(board);
    }

    boards
}

#[cfg(test)]
mod tests {
    use aoc::input::parse_test_vec;

    use super::*;

    const EXAMPLE1: &str = "\
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn test1() {
        let input = parse_test_vec(EXAMPLE1, input_transform).unwrap();
        let input = build_boards(&input);

        assert_eq!(part1(&input), 405);
        assert_eq!(part2(&input), 400);
    }
}
