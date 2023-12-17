use std::{
    collections::{HashSet, VecDeque},
    error::Error,
};

use aoc::{
    gif::Gif,
    input::{parse_input_vec, parse_test_vec},
};

const EXAMPLE1: &str = "\
.|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....";

fn main() -> Result<(), Box<dyn Error>> {
    let input = parse_test_vec(EXAMPLE1, input_transform).unwrap();

    render(&input, "vis/day16ex.gif")?;

    let input = parse_input_vec(16, input_transform)?;

    render(&input, "vis/day16.gif")?;

    Ok(())
}

fn render(map: &[InputEnt], file: &str) -> Result<(), Box<dyn Error>> {
    let mut max_result = 0;
    let mut best = None;

    let mut energise_max = |x, y, dir| {
        let result = energise(map, x, y, dir);

        if result > max_result {
            max_result = result;
            best = Some((x, y, dir));
        }
    };

    // Top and bottom rows
    for x in 0..(map[0].len()) {
        energise_max(x, 0, Dir::S);
        energise_max(x, map.len() - 1, Dir::N);
    }

    // Left and Right columns
    for y in 0..(map.len()) {
        energise_max(0, y, Dir::E);
        energise_max(map[0].len() - 1, y, Dir::W);
    }

    // Render the best path
    if let Some((x, y, dir)) = best {
        render_path(file, map, x, y, dir)?;
    }

    Ok(())
}

const CELLSIZE: usize = 5;
const MAX_INTENSITY: u8 = 8;

type CellState = [u8; 4];

fn render_path(
    file: &str,
    map: &[InputEnt],
    x: usize,
    y: usize,
    dir: Dir,
) -> Result<(), Box<dyn Error>> {
    // Create palette
    let mut palette = vec![
        [0, 0, 0],       // Black
        [128, 128, 255], // Mirror
        [255, 128, 128], // Splitter
        [0, 0, 0],       // Filler
        [0, 0, 0],       // Filler
        [0, 0, 0],       // Filler
        [0, 0, 0],       // Filler
        [0, 0, 0],       // Filler
    ];

    // Shades of white
    for i in 0u32..(MAX_INTENSITY as u32) {
        let component = (127 + ((i * 128) / MAX_INTENSITY as u32)) as u8;
        palette.push([component, component, component]);
    }

    // Create GIF
    let mut gif = Gif::new(
        file,
        &palette,
        (map[0].len() * CELLSIZE) as u16,
        (map.len() * CELLSIZE) as u16,
        1,
        1,
    )?;

    // Create cell states
    let mut cell_state: Vec<Vec<CellState>> =
        vec![vec![CellState::default(); map[0].len()]; map.len()];

    // Create gif frame
    let mut frame = gif.empty_frame();

    for y in 0..(map.len()) {
        for x in 0..(map[0].len()) {
            draw_cell(&mut frame, map, &cell_state, x, y);
        }
    }

    // Create queue
    let mut queue = VecDeque::new();

    // Add initial position and direction
    queue.push_back(((x, y), dir, 1));

    // Last draw depth
    let mut last_depth = 0;

    // Get next queue entry
    while let Some(((x, y), dir, depth)) = queue.pop_front() {
        if depth > last_depth {
            gif.draw_frame(frame.clone(), 1)?;
            last_depth = depth;
        }

        // Add intensity
        if !add_intensity(&mut frame, map, &mut cell_state, x, y, dir.opposite()) {
            continue;
        }

        match map[y][x] {
            State::Empty => {
                // Continue on this path
                add_intensity(&mut frame, map, &mut cell_state, x, y, dir);
                if let Some((nx, ny)) = add_dir(map, x, y, &dir) {
                    queue.push_back(((nx, ny), dir, depth + 1));
                }
            }
            State::MirrorNESW => {
                // Work out new direction
                let new_dir = match dir {
                    Dir::N => Dir::E,
                    Dir::E => Dir::N,
                    Dir::S => Dir::W,
                    Dir::W => Dir::S,
                };

                // Move in new direction
                add_intensity(&mut frame, map, &mut cell_state, x, y, new_dir);
                if let Some((nx, ny)) = add_dir(map, x, y, &new_dir) {
                    queue.push_back(((nx, ny), new_dir, depth + 1));
                }
            }
            State::MirrorNWSE => {
                // Work out new direction
                let new_dir = match dir {
                    Dir::N => Dir::W,
                    Dir::E => Dir::S,
                    Dir::S => Dir::E,
                    Dir::W => Dir::N,
                };

                // Move in new direction
                add_intensity(&mut frame, map, &mut cell_state, x, y, new_dir);
                if let Some((nx, ny)) = add_dir(map, x, y, &new_dir) {
                    queue.push_back(((nx, ny), new_dir, depth + 1));
                }
            }
            State::SplitterHoriz => match dir {
                Dir::E | Dir::W => {
                    // Continue on this path
                    add_intensity(&mut frame, map, &mut cell_state, x, y, dir);
                    if let Some((nx, ny)) = add_dir(map, x, y, &dir) {
                        queue.push_back(((nx, ny), dir, depth + 1));
                    }
                }
                Dir::S | Dir::N => {
                    // Split east
                    let dir1 = Dir::E;

                    add_intensity(&mut frame, map, &mut cell_state, x, y, dir1);
                    if let Some((nx, ny)) = add_dir(map, x, y, &dir1) {
                        queue.push_back(((nx, ny), dir1, depth + 1));
                    }

                    // Split west
                    let dir2 = Dir::W;

                    add_intensity(&mut frame, map, &mut cell_state, x, y, dir2);
                    if let Some((nx, ny)) = add_dir(map, x, y, &dir2) {
                        queue.push_back(((nx, ny), dir2, depth + 1));
                    }
                }
            },
            State::SplitterVert => match dir {
                Dir::S | Dir::N => {
                    // Continue on this path
                    add_intensity(&mut frame, map, &mut cell_state, x, y, dir);
                    if let Some((nx, ny)) = add_dir(map, x, y, &dir) {
                        queue.push_back(((nx, ny), dir, depth + 1));
                    }
                }
                Dir::E | Dir::W => {
                    // Split north
                    let dir1 = Dir::N;

                    add_intensity(&mut frame, map, &mut cell_state, x, y, dir1);
                    if let Some((nx, ny)) = add_dir(map, x, y, &dir1) {
                        queue.push_back(((nx, ny), dir1, depth + 1));
                    }

                    // Split south
                    let dir2 = Dir::S;

                    add_intensity(&mut frame, map, &mut cell_state, x, y, dir2);
                    if let Some((nx, ny)) = add_dir(map, x, y, &dir2) {
                        queue.push_back(((nx, ny), dir2, depth + 1));
                    }
                }
            },
        };
    }

    gif.draw_frame(frame.clone(), 1)?;

    gif.delay(1000)?;

    Ok(())
}

fn add_intensity(
    frame: &mut [Vec<u8>],
    map: &[InputEnt],
    cell_state: &mut [Vec<CellState>],
    x: usize,
    y: usize,
    dir: Dir,
) -> bool {
    let state = &mut cell_state[y][x][dir as usize];

    if *state == MAX_INTENSITY {
        return false;
    }
    *state += 1;

    draw_cell(frame, map, cell_state, x, y);

    true
}

fn draw_cell(
    frame: &mut [Vec<u8>],
    map: &[InputEnt],
    cell_state: &[Vec<CellState>],
    x: usize,
    y: usize,
) {
    let gx = x * CELLSIZE;
    let gy = y * CELLSIZE;

    let state = &cell_state[y][x];

    let mut edges = Vec::new();

    // North beam
    let intens = state[Dir::N as usize];

    if intens > 0 {
        edges.push((intens, Dir::N));
    }

    // East beam
    let intens = state[Dir::E as usize];

    if intens > 0 {
        edges.push((intens, Dir::E));
    }

    // South beam
    let intens = state[Dir::S as usize];

    if intens > 0 {
        edges.push((intens, Dir::S));
    }

    // West beam
    let intens = state[Dir::W as usize];

    if intens > 0 {
        edges.push((intens, Dir::W));
    }

    edges.sort();

    for (intens, edge) in edges {
        match edge {
            Dir::N => draw_bmp(
                frame,
                gx,
                gy,
                7 + intens,
                &[
                    [0, 1, 1, 1, 0],
                    [0, 1, 1, 1, 0],
                    [0, 1, 1, 1, 0],
                    [0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0],
                ],
            ),
            Dir::E => draw_bmp(
                frame,
                gx,
                gy,
                7 + intens,
                &[
                    [0, 0, 0, 0, 0],
                    [0, 0, 1, 1, 1],
                    [0, 0, 1, 1, 1],
                    [0, 0, 1, 1, 1],
                    [0, 0, 0, 0, 0],
                ],
            ),
            Dir::S => draw_bmp(
                frame,
                gx,
                gy,
                7 + intens,
                &[
                    [0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0],
                    [0, 1, 1, 1, 0],
                    [0, 1, 1, 1, 0],
                    [0, 1, 1, 1, 0],
                ],
            ),
            Dir::W => draw_bmp(
                frame,
                gx,
                gy,
                7 + intens,
                &[
                    [0, 0, 0, 0, 0],
                    [1, 1, 1, 0, 0],
                    [1, 1, 1, 0, 0],
                    [1, 1, 1, 0, 0],
                    [0, 0, 0, 0, 0],
                ],
            ),
        }
    }

    match map[y][x] {
        State::Empty => (),
        State::MirrorNESW => draw_bmp(
            frame,
            gx,
            gy,
            1,
            &[
                [0, 0, 0, 1, 1],
                [0, 0, 1, 1, 1],
                [0, 1, 1, 1, 0],
                [1, 1, 1, 0, 0],
                [1, 1, 0, 0, 0],
            ],
        ),
        State::MirrorNWSE => draw_bmp(
            frame,
            gx,
            gy,
            1,
            &[
                [1, 1, 0, 0, 0],
                [1, 1, 1, 0, 0],
                [0, 1, 1, 1, 0],
                [0, 0, 1, 1, 1],
                [0, 0, 0, 1, 1],
            ],
        ),
        State::SplitterHoriz => draw_bmp(
            frame,
            gx,
            gy,
            2,
            &[
                [0, 0, 0, 0, 0],
                [0, 1, 0, 1, 0],
                [1, 1, 1, 1, 1],
                [0, 1, 0, 1, 0],
                [0, 0, 0, 0, 0],
            ],
        ),
        State::SplitterVert => draw_bmp(
            frame,
            gx,
            gy,
            2,
            &[
                [0, 0, 1, 0, 0],
                [0, 1, 1, 1, 0],
                [0, 0, 1, 0, 0],
                [0, 1, 1, 1, 0],
                [0, 0, 1, 0, 0],
            ],
        ),
    }
}

fn draw_bmp(frame: &mut [Vec<u8>], gx: usize, gy: usize, colour: u8, bmp: &[[u8; 5]; 5]) {
    for (y, row) in bmp.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            if *col != 0 {
                frame[gy + y][gx + x] = colour;
            }
        }
    }
}

fn energise(map: &[InputEnt], x: usize, y: usize, dir: Dir) -> u64 {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    // Add initial position and direction
    queue.push_back((x, y, dir));

    // Get next queue entry
    while let Some((x, y, dir)) = queue.pop_front() {
        // Build hash set entry
        let visited_ent = (x, y, dir);

        // Already been here travelling in this direction?
        if visited.contains(&visited_ent) {
            continue;
        }

        // Add to visited
        visited.insert(visited_ent);

        match map[y][x] {
            State::Empty => {
                // Continue on this path
                if let Some((x, y)) = add_dir(map, x, y, &dir) {
                    queue.push_back((x, y, dir));
                }
            }
            State::MirrorNESW => {
                // Work out new direction
                let new_dir = match dir {
                    Dir::N => Dir::E,
                    Dir::E => Dir::N,
                    Dir::S => Dir::W,
                    Dir::W => Dir::S,
                };

                // Move in new direction
                if let Some((x, y)) = add_dir(map, x, y, &new_dir) {
                    queue.push_back((x, y, new_dir));
                }
            }
            State::MirrorNWSE => {
                // Work out new direction
                let new_dir = match dir {
                    Dir::N => Dir::W,
                    Dir::E => Dir::S,
                    Dir::S => Dir::E,
                    Dir::W => Dir::N,
                };

                // Move in new direction
                if let Some((x, y)) = add_dir(map, x, y, &new_dir) {
                    queue.push_back((x, y, new_dir));
                }
            }
            State::SplitterHoriz => match dir {
                Dir::E | Dir::W => {
                    // Continue on this path
                    if let Some((x, y)) = add_dir(map, x, y, &dir) {
                        queue.push_back((x, y, dir));
                    }
                }
                Dir::S | Dir::N => {
                    // Split east
                    let dir1 = Dir::E;

                    if let Some((x, y)) = add_dir(map, x, y, &dir1) {
                        queue.push_back((x, y, dir1));
                    }

                    // Split west
                    let dir2 = Dir::W;

                    if let Some((x, y)) = add_dir(map, x, y, &dir2) {
                        queue.push_back((x, y, dir2));
                    }
                }
            },
            State::SplitterVert => match dir {
                Dir::S | Dir::N => {
                    // Continue on this path
                    if let Some((x, y)) = add_dir(map, x, y, &dir) {
                        queue.push_back((x, y, dir));
                    }
                }
                Dir::E | Dir::W => {
                    // Split north
                    let dir1 = Dir::N;

                    if let Some((x, y)) = add_dir(map, x, y, &dir1) {
                        queue.push_back((x, y, dir1));
                    }

                    // Split south
                    let dir2 = Dir::S;

                    if let Some((x, y)) = add_dir(map, x, y, &dir2) {
                        queue.push_back((x, y, dir2));
                    }
                }
            },
        };
    }

    // Build set of visited locations
    let visited_set = visited
        .iter()
        .map(|(x, y, _)| (x, y))
        .collect::<HashSet<_>>();

    visited_set.len() as u64
}

fn add_dir(map: &[InputEnt], x: usize, y: usize, dir: &Dir) -> Option<(usize, usize)> {
    // Get movement
    let (xadd, yadd) = dir.movement();

    // Work out new position
    let new_x = x as isize + xadd;
    let new_y = y as isize + yadd;

    // Bounds check new position
    if new_x >= 0 && (new_x as usize) < map[0].len() && new_y >= 0 && (new_y as usize) < map.len() {
        // Within map bounds
        Some((new_x as usize, new_y as usize))
    } else {
        // Outside map
        None
    }
}

#[derive(Debug)]
enum State {
    Empty,
    MirrorNESW,
    MirrorNWSE,
    SplitterHoriz,
    SplitterVert,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Dir {
    N = 0,
    E,
    S,
    W,
}

impl Dir {
    fn movement(&self) -> (isize, isize) {
        match self {
            Dir::N => (0, -1),
            Dir::E => (1, 0),
            Dir::S => (0, 1),
            Dir::W => (-1, 0),
        }
    }

    fn opposite(&self) -> Self {
        match self {
            Dir::N => Dir::S,
            Dir::E => Dir::W,
            Dir::S => Dir::N,
            Dir::W => Dir::E,
        }
    }
}

// Input parsing

type InputEnt = Vec<State>;

fn input_transform(line: String) -> InputEnt {
    line.chars()
        .map(|c| match c {
            '.' => State::Empty,
            '/' => State::MirrorNESW,
            '\\' => State::MirrorNWSE,
            '-' => State::SplitterHoriz,
            '|' => State::SplitterVert,
            _ => panic!("Unvalid character"),
        })
        .collect::<Vec<_>>()
}
