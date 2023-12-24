use std::{
    cmp::max,
    collections::{hash_map::Entry, HashMap, HashSet, VecDeque},
    error::Error,
};

use aoc::input::parse_input_vec;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(23, input_transform)?;

    // Run parts
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

fn part1(map: &[MapRow]) -> u64 {
    let mut longest = 0;

    let (sx, sy, ex, ey) = find_exits(map);

    let mut queue = VecDeque::new();
    let mut visited = HashMap::new();

    queue.push_back((sx, sy, Dir::S, 0));

    while let Some((x, y, dir, steps)) = queue.pop_back() {
        if x == ex && y == ey {
            longest = max(longest, steps);
            continue;
        }

        let mut move_to = |x, y, dir, steps| {
            let v = visited.entry((x, y));

            match v {
                Entry::Occupied(mut v) => {
                    if *v.get() >= steps {
                        return;
                    }

                    *v.get_mut() = steps;
                }
                Entry::Vacant(v) => {
                    v.insert(steps);
                }
            }

            queue.push_back((x, y, dir, steps));
        };

        // North
        if !matches!(dir, Dir::S) && y > 0 {
            match map[y - 1][x] {
                Tile::Path | Tile::SlopeN => move_to(x, y - 1, Dir::N, steps + 1),
                _ => (),
            }
        }

        // East
        if !matches!(dir, Dir::W) {
            match map[y][x + 1] {
                Tile::Path | Tile::SlopeE => move_to(x + 1, y, Dir::E, steps + 1),
                _ => (),
            }
        }

        // South
        if !matches!(dir, Dir::N) {
            match map[y + 1][x] {
                Tile::Path | Tile::SlopeS => move_to(x, y + 1, Dir::S, steps + 1),
                _ => (),
            }
        }

        // West
        if !matches!(dir, Dir::E) {
            match map[y][x - 1] {
                Tile::Path | Tile::SlopeW => move_to(x - 1, y, Dir::W, steps + 1),
                _ => (),
            }
        }
    }

    longest
}

fn part2(map: &[MapRow]) -> u64 {
    // Find nodes
    let (sx, sy, ex, ey) = find_exits(map);

    let mut nodes = HashSet::new();

    nodes.insert((sx, sy));
    nodes.insert((ex, ey));

    for y in 1..(map.len() - 1) {
        for x in 1..(map[0].len() - 1) {
            if matches!(map[y][x], Tile::Forest) {
                continue;
            }

            let mut moves = Vec::new();

            let mut move_to = |x: usize, y: usize| {
                if !matches!(map[y][x], Tile::Forest) {
                    moves.push((x, y));
                }
            };

            move_to(x, y - 1);
            move_to(x + 1, y);
            move_to(x, y + 1);
            move_to(x - 1, y);

            if moves.len() > 2 {
                nodes.insert((x, y));
            }
        }
    }

    // Build edges
    let mut edges: HashMap<Node, Vec<(Node, u64)>> = HashMap::new();

    for (x, y) in nodes.iter() {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();

        queue.push_back((*x, *y, 0, *x, *y));
        visited.insert((*x, *y));

        while let Some((x, y, steps, nx, ny)) = queue.pop_front() {
            if (x != nx || y != ny) && nodes.contains(&(x, y)) {
                // On a node - add edge
                let ent = ((x, y), steps);

                edges
                    .entry((nx, ny))
                    .and_modify(|e| e.push(ent))
                    .or_insert(vec![ent]);

                continue;
            }

            let mut move_to = |x: usize, y: usize| {
                if !matches!(map[y][x], Tile::Forest) && !visited.contains(&(x, y)) {
                    queue.push_back((x, y, steps + 1, nx, ny));
                    visited.insert((x, y));
                }
            };

            if y > 0 {
                move_to(x, y - 1);
            }

            move_to(x + 1, y);

            if y < map.len() - 1 {
                move_to(x, y + 1);
            }

            move_to(x - 1, y);
        }
    }

    let state = State {
        x: sx,
        y: sy,
        steps: 0,
        visited: HashSet::new(),
    };

    find_longest(state, ex, ey, &edges)
}

type Node = (usize, usize);

struct State {
    x: usize,
    y: usize,
    steps: u64,
    visited: HashSet<Node>,
}

fn find_longest(
    mut state: State,
    ex: usize,
    ey: usize,
    edges: &HashMap<Node, Vec<(Node, u64)>>,
) -> u64 {
    state.visited.insert((state.x, state.y));

    edges
        .get(&(state.x, state.y))
        .unwrap()
        .iter()
        .filter_map(|&((x, y), steps)| {
            if state.visited.contains(&(x, y)) {
                None
            } else {
                let steps = state.steps + steps;

                Some(if x == ex && y == ey {
                    steps
                } else {
                    find_longest(
                        State {
                            x,
                            y,
                            steps,
                            visited: state.visited.clone(),
                        },
                        ex,
                        ey,
                        edges,
                    )
                })
            }
        })
        .max()
        .unwrap_or(0)
}

fn find_exits(map: &[MapRow]) -> (usize, usize, usize, usize) {
    let ey = map.len() - 1;

    (
        map[0].iter().position(|t| matches!(t, Tile::Path)).unwrap(),
        0,
        map[ey]
            .iter()
            .position(|t| matches!(t, Tile::Path))
            .unwrap(),
        ey,
    )
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Dir {
    N,
    E,
    S,
    W,
}

// Input parsing

enum Tile {
    Path,
    Forest,
    SlopeN,
    SlopeW,
    SlopeE,
    SlopeS,
}

type MapRow = Vec<Tile>;

fn input_transform(line: String) -> MapRow {
    line.chars()
        .map(|c| match c {
            '.' => Tile::Path,
            '#' => Tile::Forest,
            '^' => Tile::SlopeN,
            '<' => Tile::SlopeW,
            '>' => Tile::SlopeE,
            'v' => Tile::SlopeS,
            _ => panic!("Invalid tile"),
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use aoc::input::parse_test_vec;

    use super::*;

    const EXAMPLE1: &str = "\
#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";

    #[test]
    fn test1() {
        let input = parse_test_vec(EXAMPLE1, input_transform).unwrap();
        assert_eq!(part1(&input), 94);
        assert_eq!(part2(&input), 154);
    }
}
