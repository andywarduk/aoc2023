use std::error::Error;

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
    let mut result = 0;

    for line in input {
        result += piece_solutions(&line.pieces, &line.clues);
    }

    result
}

fn part2(input: &[InputEnt]) -> u64 {
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

            InputEnt {
                pieces: new_pieces,
                clues: new_clues,
            }
        })
        .collect::<Vec<InputEnt>>();

    let mut result = 0;

    for line in input {
        result += piece_solutions(&line.pieces, &line.clues);
        println!("{result}");
    }

    result
}

fn piece_solutions(pieces: &[SpringState], clues: &[u8]) -> u64 {
    // Build match pattern
    let (pattern_len, pattern) = clues_to_pattern(pieces, clues);

    let mut solutions = 0;
    solve(pieces.to_vec(), 0, pattern_len, &pattern, 0, &mut solutions);

    solutions
}

fn clues_to_pattern(pieces: &[SpringState], clues: &[u8]) -> (usize, Vec<PatternElem>) {
    let mut pattern = Vec::new();

    pattern.push(PatternElem::MaybeWorking);

    for c in clues {
        for _ in 0..*c {
            pattern.push(PatternElem::Broken);
        }
        pattern.push(PatternElem::Working);
        pattern.push(PatternElem::MaybeWorking);
    }

    pattern.pop();
    pattern.pop();

    let pattern_len = pattern.len();

    if !matches!(pieces[pieces.len() - 1], SpringState::Broken) {
        pattern.push(PatternElem::MaybeWorking);
    }

    (pattern_len, pattern)
}

fn solve(
    pieces: Vec<SpringState>,
    mut start: usize,
    pattern_len: usize,
    pattern: &[PatternElem],
    pattern_elem: usize,
    solutions: &mut u64,
) {
    let mut pieces = pieces.to_vec();
    let mut elem = pattern_elem;

    match check_sol(&mut pieces, &mut start, pattern, &mut elem) {
        None => {
            if elem >= pattern_len {
                #[cfg(test)]
                println!("SOLVED: {}", print_state(&pieces));
                *solutions += 1;
            }
        }
        Some(true) => {
            let mut pieces_rec = pieces.clone();
            pieces_rec[start] = SpringState::Broken;
            solve(pieces_rec, start, pattern_len, pattern, elem, solutions);

            let mut pieces_rec = pieces.clone();
            pieces_rec[start] = SpringState::Working;
            solve(pieces_rec, start, pattern_len, pattern, elem, solutions);
        }
        Some(false) => {
            #[cfg(test)]
            println!("No match")
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum SpringState {
    Unsolved,
    Working,
    Broken,
}

#[derive(Debug)]
enum PatternElem {
    Working,      // Matches exactly one working spring
    Broken,       // Matches exactly one broken spring
    MaybeWorking, // Matches zero or more working springs
}

#[derive(Debug)]
enum Match {
    None,
    Matched,
    Maybe,
    Choice,
    Set(SpringState),
}

fn check_sol(
    state: &mut [SpringState],
    start: &mut usize,
    pattern: &[PatternElem],
    pattern_elem: &mut usize,
) -> Option<bool> {
    #[cfg(test)]
    println!(
        "Checking {} ({start}) against {pattern:?} ({pattern_elem})",
        print_state(state)
    );

    let matched = state.iter_mut().skip(*start).find_map(|s| {
        if *pattern_elem >= pattern.len() {
            return Some(false);
        }

        #[cfg(test)]
        print!(
            "{s:?} vs {:?} ({}) : ",
            pattern[*pattern_elem], *pattern_elem
        );

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
                    if *pattern_elem + 1 == pattern.len() {
                        Match::None
                    } else {
                        *pattern_elem += 1;

                        if matches!(pattern[*pattern_elem], PatternElem::Broken) {
                            Match::Matched
                        } else {
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

        #[cfg(test)]
        println!("{matched:?}");

        if let Match::Set(spring_state) = matched {
            *s = spring_state;
            matched = Match::Matched;
        };

        match matched {
            Match::None => Some(false),
            Match::Matched => {
                *start += 1;
                *pattern_elem += 1;
                None
            }
            Match::Maybe => {
                *start += 1;
                None
            }
            Match::Choice => Some(true),
            _ => unreachable!(),
        }
    });

    #[cfg(test)]
    println!(
        " -> {matched:?}, {} {}, {}",
        *start,
        *pattern_elem,
        print_state(state)
    );

    matched
}

#[cfg(test)]
fn print_state(state: &[SpringState]) -> String {
    state
        .iter()
        .map(|s| match s {
            SpringState::Broken => '#',
            SpringState::Working => '.',
            SpringState::Unsolved => '?',
        })
        .collect::<String>()
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

        let mut solutions = 0;
        solve(
            line.pieces.to_vec(),
            0,
            pattern_len,
            &pattern,
            0,
            &mut solutions,
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
    fn test1() {
        let input = parse_test_vec(EXAMPLE1, input_transform).unwrap();
        assert_eq!(part1(&input), 21);
        assert_eq!(part2(&input), 525152);
    }
}
