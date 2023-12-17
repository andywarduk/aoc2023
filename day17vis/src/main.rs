use std::{
    cmp::Ordering,
    collections::{hash_map::Entry, BinaryHeap, HashMap},
    error::Error,
};

use aoc::{gif::Gif, input::parse_input_vec};

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let map = parse_input_vec(17, input_transform)?;

    // Create palette
    let mut palette = Vec::new();

    let component = |i: u16| 64u8 + (((255 - 64) * i) / 9) as u8;

    // Blues
    for i in 0..=9 {
        palette.push([0, 0, component(i)])
    }

    // Yellows
    for i in 0..=9 {
        palette.push([component(i), component(i), 0])
    }

    // Reds
    for i in 0..=9 {
        palette.push([component(i), 0, 0])
    }

    // Create gif
    let mut gif = Gif::new(
        "vis/day17.gif",
        &palette,
        map[0].len() as u16,
        map.len() as u16,
        4,
        4,
    )?;

    // Base frame
    let mut base_frame = gif.empty_frame();

    for (y, row) in map.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            base_frame[y][x] = *cell;
        }
    }

    gif.draw_frame(base_frame.clone(), 0)?;

    // Run parts
    part1(&map, &mut gif, &mut base_frame, 1)?;

    // Add delay
    gif.delay(100)?;

    part2(&map, &mut gif, &mut base_frame, 2)?;

    // Add final delay
    gif.delay(1000)?;

    Ok(())
}

fn part1(
    map: &[InputEnt],
    gif: &mut Gif,
    frame: &mut [Vec<u8>],
    colour: u8,
) -> Result<(), Box<dyn Error>> {
    solve(map, gif, frame, colour, 1, 3)
}

fn part2(
    map: &[InputEnt],
    gif: &mut Gif,
    frame: &mut [Vec<u8>],
    colour: u8,
) -> Result<(), Box<dyn Error>> {
    solve(map, gif, frame, colour, 4, 10)
}

#[derive(Debug, PartialEq, Eq)]
struct State {
    x: usize,
    y: usize,
    dir: Dir,
    len: usize,
    loss: u64,
    path: Vec<(usize, usize)>,
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
        // Get x and y adjustment for direction
        let (xadd, yadd) = match self {
            Dir::N => (0, -1),
            Dir::E => (1, 0),
            Dir::S => (0, 1),
            Dir::W => (-1, 0),
        };

        // Calculate new position
        let new_x = x as isize + xadd;
        let new_y = y as isize + yadd;

        // Bounds check
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

fn solve(
    map: &[InputEnt],
    gif: &mut Gif,
    frame: &mut [Vec<u8>],
    colour: u8,
    min_move: usize,
    max_move: usize,
) -> Result<(), Box<dyn Error>> {
    let mut best = u64::MAX; // Best loss so far
    let mut best_path = vec![]; // Best path so far

    let target_x = map[0].len() - 1; // Target x coord
    let target_y = map.len() - 1; // Target y coord

    let mut queue = BinaryHeap::new(); // Work queue
    let mut visited = HashMap::new(); // Visited hash map

    // Add start going east
    queue.push(State {
        x: 1,
        y: 0,
        dir: Dir::E,
        len: 2,
        loss: 0,
        path: vec![(0, 0)],
    });

    // Add start going south
    queue.push(State {
        x: 0,
        y: 1,
        dir: Dir::S,
        len: 2,
        loss: 0,
        path: vec![(0, 0)],
    });

    let new_path = |old_path: &Vec<(usize, usize)>, x, y| {
        let mut new_path = old_path.clone();
        new_path.push((x, y));
        new_path
    };

    // Get next work element
    while let Some(state) = queue.pop() {
        // Calculate new loss
        let new_loss = state.loss + map[state.y][state.x] as u64;

        // Reached the target?
        if state.x == target_x && state.y == target_y {
            // Yes - is loss better than we've got already?
            if new_loss < best {
                // Yes
                best = new_loss;
                best_path = new_path(&state.path, target_x, target_y);
            }

            continue;
        }

        // Already visited?
        match visited.entry((state.x, state.y, state.dir, state.len)) {
            Entry::Occupied(mut e) => {
                // Yes - check loss against previous visit
                let loss = e.get_mut();

                if *loss > new_loss {
                    // Better than previous - update
                    *loss = new_loss;
                } else {
                    // Worse than previous - skip
                    continue;
                }
            }
            Entry::Vacant(e) => {
                // No - insert
                e.insert(new_loss);
            }
        }

        // Calculate new forwards length
        let new_len = state.len + 1;

        // Allowed this many forwards?
        if new_len <= max_move {
            // Calculate new position going straight on
            if let Some((new_x, new_y)) = state.dir.apply(map, state.x, state.y) {
                // Add to queue
                queue.push(State {
                    x: new_x,
                    y: new_y,
                    dir: state.dir,
                    len: new_len,
                    loss: new_loss,
                    path: new_path(&state.path, state.x, state.y),
                });
            }
        }

        // Allowed to turn?
        if state.len >= min_move {
            // Yes

            // Get left and right directions
            let (l, r) = match state.dir {
                Dir::N => (Dir::W, Dir::E),
                Dir::E => (Dir::N, Dir::S),
                Dir::S => (Dir::E, Dir::W),
                Dir::W => (Dir::S, Dir::N),
            };

            // Calculate new position turning left
            if let Some((new_x, new_y)) = l.apply(map, state.x, state.y) {
                // Add to queue
                queue.push(State {
                    x: new_x,
                    y: new_y,
                    dir: l,
                    len: 1,
                    loss: new_loss,
                    path: new_path(&state.path, state.x, state.y),
                });
            }

            // Calculate new position turning right
            if let Some((new_x, new_y)) = r.apply(map, state.x, state.y) {
                queue.push(State {
                    x: new_x,
                    y: new_y,
                    dir: r,
                    len: 1,
                    loss: new_loss,
                    path: new_path(&state.path, state.x, state.y),
                });
            }
        }
    }

    // Animate the best path
    for (x, y) in best_path {
        frame[y][x] = (10 * colour) + map[y][x];

        gif.draw_frame(frame.to_owned(), 1)?;
    }

    Ok(())
}

// Input parsing

type InputEnt = Vec<u8>;

fn input_transform(line: String) -> InputEnt {
    line.chars().map(|c| c as u8 - b'0').collect::<Vec<_>>()
}
