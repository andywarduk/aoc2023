use std::{
    cmp::Ordering,
    collections::{hash_map::Entry, BinaryHeap, HashMap},
    error::Error,
};

use aoc::input::parse_input_vec;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(17, input_transform)?;

    // Run parts
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

fn part1(map: &[InputEnt]) -> u64 {
    solve(map, 1, 3)
}

fn part2(map: &[InputEnt]) -> u64 {
    solve(map, 4, 10)
}

#[derive(Debug, PartialEq, Eq)]
struct State {
    x: usize,
    y: usize,
    dir: Dir,
    len: usize,
    loss: u64,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .loss
            .cmp(&self.loss)
            .then_with(|| self.dir.cmp(&other.dir))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum Dir {
    S,
    E,
    N,
    W,
}

impl Dir {
    fn apply(&self, map: &[InputEnt], x: usize, y: usize) -> Option<(usize, usize)> {
        let (xadd, yadd) = match self {
            Dir::N => (0, -1),
            Dir::E => (1, 0),
            Dir::S => (0, 1),
            Dir::W => (-1, 0),
        };

        let new_x = x as isize + xadd;
        let new_y = y as isize + yadd;

        if new_x >= 0
            && (new_x as usize) < map[0].len()
            && new_y >= 0
            && (new_y as usize) < map.len()
        {
            Some((new_x as usize, new_y as usize))
        } else {
            None
        }
    }
}

fn solve(map: &[InputEnt], min_move: usize, max_move: usize) -> u64 {
    let mut best = u64::MAX;
    let target_x = map[0].len() - 1;
    let target_y = map.len() - 1;

    let mut queue = BinaryHeap::new();
    let mut visited = HashMap::new();

    queue.push(State {
        x: 1,
        y: 0,
        dir: Dir::E,
        len: 2,
        loss: 0,
    });

    queue.push(State {
        x: 0,
        y: 1,
        dir: Dir::S,
        len: 2,
        loss: 0,
    });

    while let Some(state) = queue.pop() {
        // Calculate new loss
        let new_loss = state.loss + map[state.y][state.x] as u64;

        if state.x == target_x && state.y == target_y {
            if new_loss <= best {
                best = new_loss;
            }
            continue;
        }

        // Already visited?
        match visited.entry((state.x, state.y, state.dir, state.len)) {
            Entry::Occupied(mut e) => {
                let loss = e.get_mut();

                if *loss > new_loss {
                    *loss = new_loss;
                } else {
                    continue;
                }
            }
            Entry::Vacant(e) => {
                e.insert(new_loss);
            }
        }

        let (l, r) = match state.dir {
            Dir::N => (Dir::W, Dir::E),
            Dir::E => (Dir::N, Dir::S),
            Dir::S => (Dir::E, Dir::W),
            Dir::W => (Dir::S, Dir::N),
        };

        if state.len >= min_move {
            // Calculate new position turning left
            if let Some((new_x, new_y)) = l.apply(map, state.x, state.y) {
                queue.push(State {
                    x: new_x,
                    y: new_y,
                    dir: l,
                    len: 1,
                    loss: new_loss,
                });
            }
        }

        let new_len = state.len + 1;

        if new_len <= max_move {
            // Calculate new position going straight on
            if let Some((new_x, new_y)) = state.dir.apply(map, state.x, state.y) {
                queue.push(State {
                    x: new_x,
                    y: new_y,
                    dir: state.dir,
                    len: new_len,
                    loss: new_loss,
                });
            }
        }

        if state.len >= min_move {
            // Calculate new position turning left
            if let Some((new_x, new_y)) = r.apply(map, state.x, state.y) {
                queue.push(State {
                    x: new_x,
                    y: new_y,
                    dir: r,
                    len: 1,
                    loss: new_loss,
                });
            }
        }
    }

    best
}

// Input parsing

type InputEnt = Vec<u8>;

fn input_transform(line: String) -> InputEnt {
    line.chars().map(|c| c as u8 - b'0').collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use aoc::input::parse_test_vec;

    use super::*;

    const EXAMPLE1: &str = "\
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

    #[test]
    fn test1() {
        let input = parse_test_vec(EXAMPLE1, input_transform).unwrap();
        assert_eq!(part1(&input), 102);
        assert_eq!(part2(&input), 94);
    }
}
