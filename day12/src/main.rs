use std::{collections::HashMap, error::Error};

use aoc::input::parse_input_vec;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(12, input_transform)?;

    // Run parts
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

fn part1(input: &[InputEnt]) -> u64 {
    // Process input
    input
        .iter()
        .map(|line| piece_solutions(&line.pieces, &line.clues))
        .sum()
}

fn part2(input: &[InputEnt]) -> u64 {
    // Expand inputs 5-fold
    let input = input
        .iter()
        .map(|InputEnt { pieces, clues }| {
            let mut new_pieces = Vec::new();
            let mut new_clues = Vec::new();

            for _ in 0..5 {
                for p in pieces {
                    new_pieces.push(*p);
                }
                new_pieces.push(SpringState::Unsolved);

                for c in clues {
                    new_clues.push(*c);
                }
            }
            new_pieces.pop();

            InputEnt {
                pieces: new_pieces,
                clues: new_clues,
            }
        })
        .collect::<Vec<InputEnt>>();

    // Process new input
    input
        .iter()
        .map(|line| piece_solutions(&line.pieces, &line.clues))
        .sum()
}

fn piece_solutions(pieces: &[SpringState], clues: &[u8]) -> u64 {
    // Build match pattern
    let (pattern_len, pattern) = clues_to_pattern(pieces, clues);

    // Create memo hash map
    let mut sol_map = HashMap::new();

    // Solve
    solve(pieces.to_vec(), 0, pattern_len, &pattern, 0, &mut sol_map)
}

fn clues_to_pattern(pieces: &[SpringState], clues: &[u8]) -> (usize, Vec<PatternElem>) {
    let mut pattern = Vec::new();

    // Start with maybe working
    pattern.push(PatternElem::MaybeWorking);

    for c in clues {
        // Add broken group
        for _ in 0..*c {
            pattern.push(PatternElem::Broken);
        }

        // Broken group must be followed by working
        pattern.push(PatternElem::Working);
        pattern.push(PatternElem::MaybeWorking);
    }

    // Remove last working group
    pattern.pop();
    pattern.pop();

    // Set minimum pattern match length
    let pattern_len = pattern.len();

    // Does line end with a broken spring?
    if !matches!(pieces[pieces.len() - 1], SpringState::Broken) {
        // No - add MaybeWorking to the end of the pattern
        pattern.push(PatternElem::MaybeWorking);
    }

    (pattern_len, pattern)
}

fn solve(
    pieces: Vec<SpringState>,
    piece_start: usize,
    pattern_len: usize,
    pattern: &[PatternElem],
    pattern_elem: usize,
    sol_map: &mut HashMap<(usize, usize, SpringState), u64>,
) -> u64 {
    // Check memo hash map for an existing solution
    if let Some(solutions) = sol_map.get(&(piece_start, pattern_elem, pieces[piece_start])) {
        // Found one - return it
        *solutions
    } else {
        let mut solutions = 0;
        let mut pieces = pieces.to_vec();
        let mut new_piece_start = piece_start;
        let mut new_pattern_elem = pattern_elem;

        // Check the solution so far
        match check_sol(
            &mut pieces,
            &mut new_piece_start,
            pattern,
            &mut new_pattern_elem,
        ) {
            None => {
                // Complete pattern match - check match length
                if new_pattern_elem >= pattern_len {
                    // Matched
                    solutions += 1;
                }
            }
            Some(true) => {
                // Found a choice - try with a broken spring
                let mut pieces_rec = pieces.clone();
                pieces_rec[new_piece_start] = SpringState::Broken;
                solutions += solve(
                    pieces_rec,
                    new_piece_start,
                    pattern_len,
                    pattern,
                    new_pattern_elem,
                    sol_map,
                );

                // Then try with a working spring
                let mut pieces_rec = pieces.clone();
                pieces_rec[new_piece_start] = SpringState::Working;
                solutions += solve(
                    pieces_rec,
                    new_piece_start,
                    pattern_len,
                    pattern,
                    new_pattern_elem,
                    sol_map,
                );
            }
            Some(false) => (), // No match
        }

        sol_map.insert((piece_start, pattern_elem, pieces[piece_start]), solutions);

        solutions
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum SpringState {
    Unsolved, // Unsolved spring
    Working,  // Working spring
    Broken,   // Broken spring
}

#[derive(Debug)]
enum PatternElem {
    Working,      // Matches exactly one working spring
    Broken,       // Matches exactly one broken spring
    MaybeWorking, // Matches zero or more working springs
}

#[derive(Debug)]
enum Match {
    None,             // No match
    Matched,          // Matched exactly
    Maybe,            // Matched maybe
    Choice,           // Choice found
    Set(SpringState), // Set state (= Matched)
}

fn check_sol(
    state: &mut [SpringState],
    start: &mut usize,
    pattern: &[PatternElem],
    pattern_elem: &mut usize,
) -> Option<bool> {
    let matched = state.iter_mut().skip(*start).find_map(|s| {
        // Check pattern bounds
        if *pattern_elem >= pattern.len() {
            return Some(false);
        }

        // Match pattern against spring state
        let mut matched = match pattern[*pattern_elem] {
            PatternElem::Working => match s {
                SpringState::Working => Match::Matched,
                SpringState::Unsolved => Match::Set(SpringState::Working),
                SpringState::Broken => Match::None,
            },
            PatternElem::MaybeWorking => match s {
                SpringState::Unsolved => Match::Choice,
                SpringState::Working => Match::Maybe,
                SpringState::Broken => {
                    // Got a broken spring for MaybeWorking - check advance
                    if *pattern_elem + 1 == pattern.len() {
                        // Pattern exhausted
                        Match::None
                    } else {
                        // Move to next pattern element
                        *pattern_elem += 1;

                        // Is next pattern element a broken spring?
                        if matches!(pattern[*pattern_elem], PatternElem::Broken) {
                            // Yes - matched
                            Match::Matched
                        } else {
                            // No - no match
                            Match::None
                        }
                    }
                }
            },
            PatternElem::Broken => match s {
                SpringState::Broken => Match::Matched,
                SpringState::Unsolved => Match::Set(SpringState::Broken),
                SpringState::Working => Match::None,
            },
        };

        // Need to set the piece?
        if let Match::Set(spring_state) = matched {
            *s = spring_state;
            matched = Match::Matched;
        };

        // Check match state
        match matched {
            Match::None => Some(false),
            Match::Matched => {
                // Matched - advance state and pattern
                *start += 1;
                *pattern_elem += 1;
                None
            }
            Match::Maybe => {
                // Partial match - advance state
                *start += 1;
                None
            }
            Match::Choice => {
                // Choice found
                Some(true)
            }
            _ => unreachable!(),
        }
    });

    matched
}

// Input parsing

struct InputEnt {
    pieces: Vec<SpringState>,
    clues: Vec<u8>,
}

fn input_transform(line: String) -> InputEnt {
    let mut split = line.split_ascii_whitespace();

    let pieces = split.next().unwrap();

    let pieces = pieces
        .chars()
        .map(|c| match c {
            '?' => SpringState::Unsolved,
            '#' => SpringState::Broken,
            '.' => SpringState::Working,
            _ => panic!("Invalid char"),
        })
        .collect::<Vec<SpringState>>();

    let clues = split.next().unwrap();

    let clues = clues
        .split(',')
        .map(|c| c.parse::<u8>().unwrap())
        .collect::<Vec<u8>>();

    InputEnt { pieces, clues }
}

#[cfg(test)]
mod tests {
    use aoc::input::parse_test_vec;

    use super::*;

    const EXAMPLE1: &str = "\
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    fn test_solve(input: &str, expected: u64) {
        let line = input_transform(input.to_string());

        let (pattern_len, pattern) = clues_to_pattern(&line.pieces, &line.clues);

        let mut sol_map = HashMap::new();

        let solutions = solve(
            line.pieces.to_vec(),
            0,
            pattern_len,
            &pattern,
            0,
            &mut sol_map,
        );

        assert_eq!(expected, solutions);
    }

    #[test]
    fn test_solve1() {
        test_solve("???.### 1,1,3", 1)
    }

    #[test]
    fn test_solve2() {
        test_solve(".??..??...?##. 1,1,3", 4)
    }

    #[test]
    fn test_solve3() {
        test_solve("?#?#?#?#?#?#?#? 1,3,1,6", 1)
    }

    #[test]
    fn test_solve4() {
        test_solve("????.#...#... 4,1,1", 1)
    }

    #[test]
    fn test_solve5() {
        test_solve("????.######..#####. 1,6,5", 4)
    }

    #[test]
    fn test_solve6() {
        test_solve("?###???????? 3,2,1", 10)
    }

    #[test]
    fn test_solve7() {
        test_solve("?###??????????###??????????###??????????###??????????###???????? 3,2,1,3,2,1,3,2,1,3,2,1,3,2,1", 506250)
    }

    #[test]
    fn test1() {
        let input = parse_test_vec(EXAMPLE1, input_transform).unwrap();
        assert_eq!(part1(&input), 21);
    }

    #[test]
    fn test2() {
        let input = parse_test_vec(EXAMPLE1, input_transform).unwrap();
        assert_eq!(part2(&input), 525152);
    }
}
