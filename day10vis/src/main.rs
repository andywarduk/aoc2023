use std::{
    cmp::max,
    collections::{HashSet, VecDeque},
    error::Error,
};

use colorgrad::CustomGradient;

use aoc::{gif::Gif, input::parse_input_vec};

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let mut input = parse_input_vec(10, input_transform)?;
    let (x, y) = find_start(&mut input);

    // Run parts
    visualise(&input, x, y)?;

    Ok(())
}

fn find_start(map: &mut [MapRow]) -> (usize, usize) {
    // Find start
    let (x, y) = map
        .iter()
        .enumerate()
        .find_map(|(y, l)| {
            l.iter()
                .enumerate()
                .find_map(|(x, c)| if *c == Pipe::Start { Some(x) } else { None })
                .map(|x| (x, y))
        })
        .unwrap();

    let mut dirs = Vec::new();

    // Which directions?
    match map[y - 1][x] {
        Pipe::NS | Pipe::SE | Pipe::SW => dirs.push(Dir::N),
        _ => (),
    }
    match map[y + 1][x] {
        Pipe::NS | Pipe::NE | Pipe::NW => dirs.push(Dir::S),
        _ => (),
    }
    match map[y][x - 1] {
        Pipe::EW | Pipe::SE | Pipe::NE => dirs.push(Dir::W),
        _ => (),
    }
    match map[y][x + 1] {
        Pipe::EW | Pipe::SW | Pipe::NW => dirs.push(Dir::E),
        _ => (),
    }

    dirs.sort();

    let start_pipe = match (dirs[0], dirs[1]) {
        (Dir::N, Dir::E) => Pipe::NE,
        (Dir::N, Dir::S) => Pipe::NS,
        (Dir::N, Dir::W) => Pipe::NW,
        (Dir::E, Dir::S) => Pipe::SE,
        (Dir::E, Dir::W) => Pipe::EW,
        (Dir::S, Dir::W) => Pipe::SW,
        _ => panic!("Unable to find start pipe"),
    };

    map[y][x] = start_pipe;

    (x, y)
}

const CELLDIM: usize = 3;
const RANGE_COLS: usize = 240;
const DEPTH_DRAW: usize = 30;

fn visualise(map: &[MapRow], start_x: usize, start_y: usize) -> Result<(), Box<dyn Error>> {
    // Build palette
    let mut palette: Vec<[u8; 3]> = vec![[0, 0, 0], [255, 255, 255], [128, 128, 128], [0, 0, 255]];
    let range_start = palette.len();

    let grad = CustomGradient::new()
        .html_colors(&["seagreen", "gold", "deeppink"])
        .build()?;

    for c in 0..=RANGE_COLS {
        let col = grad.at(c as f64 / RANGE_COLS as f64).to_rgba8();
        palette.push([col[0], col[1], col[2]]);
    }

    // Create GIF
    let mut gif = Gif::new(
        "vis/day10.gif",
        palette.as_slice(),
        (map[0].len() * CELLDIM) as u16,
        (map.len() * CELLDIM) as u16,
        2,
        2,
    )?;

    // Draw base frame
    let mut base_frame = gif.empty_frame();

    for (y, row) in map.iter().enumerate() {
        for (x, p) in row.iter().enumerate() {
            drawpipe(&mut base_frame, p, x, y, 1);
        }
    }

    // Output base frame
    gif.draw_frame(base_frame.clone(), 0)?;

    // Start position queue
    let mut queue = VecDeque::new();

    if matches!(map[start_y][start_x], Pipe::NS | Pipe::NE | Pipe::NW) {
        queue.push_back((start_x, start_y, Dir::S, 1));
    }
    if matches!(map[start_y][start_x], Pipe::NE | Pipe::SE | Pipe::EW) {
        queue.push_back((start_x, start_y, Dir::W, 1));
    }
    if matches!(map[start_y][start_x], Pipe::SE | Pipe::SW | Pipe::NS) {
        queue.push_back((start_x, start_y, Dir::N, 1));
    }
    if matches!(map[start_y][start_x], Pipe::NW | Pipe::SW | Pipe::EW) {
        queue.push_back((start_x, start_y, Dir::E, 1));
    }

    // Walk state
    let mut last_depth = 0;
    let mut visited = Vec::new();
    let mut visited_set = HashSet::new();

    // Add start to visited
    visited.push((start_x, start_y));
    visited_set.insert((start_x, start_y));

    // Walk the pipes
    while let Some((mut x, mut y, mut dir, depth)) = queue.pop_front() {
        if depth > last_depth + DEPTH_DRAW {
            // Draw a frame
            draw_path(map, &mut gif, base_frame.clone(), range_start, &visited)?;

            last_depth = depth;
        }

        // Move to next pipe
        dir = map[y][x].next_dir(dir);

        (x, y) = match dir {
            Dir::N => (x, y - 1),
            Dir::S => (x, y + 1),
            Dir::E => (x + 1, y),
            Dir::W => (x - 1, y),
        };

        // Already visited?
        if visited_set.contains(&(x, y)) {
            continue;
        }

        // Add to visited
        visited.push((x, y));
        visited_set.insert((x, y));

        // Add to work queue
        queue.push_back((x, y, dir, depth + 1));
    }

    // Called when a pipe is being crossed
    let cross_pipe = |pipe_count: &mut usize, in_dir: &mut Option<Dir>| {
        *pipe_count += 1;
        *in_dir = None
    };

    // Called when coming in in a given direction
    let in_out = |dir: Dir, pipe_count: &mut usize, in_dir: &mut Option<Dir>| match in_dir {
        Some(cur_dir) => {
            if *cur_dir == dir {
                // In and out same direction
                *in_dir = None
            } else {
                // In and out in opposite directions
                cross_pipe(pipe_count, in_dir)
            }
        }
        None => {
            // In in a direction
            *in_dir = Some(dir)
        }
    };

    // Find uncontained squares
    let mut frame = base_frame.clone();

    map.iter().enumerate().for_each(|(y, l)| {
        let mut pipe_count = 0;
        let mut in_dir = None;

        for (x, c) in l.iter().enumerate() {
            // Square part of the visited pipe?
            if visited.contains(&(x, y)) {
                match c {
                    Pipe::NS => cross_pipe(&mut pipe_count, &mut in_dir),
                    Pipe::NE | Pipe::NW => in_out(Dir::N, &mut pipe_count, &mut in_dir),
                    Pipe::SW | Pipe::SE => in_out(Dir::S, &mut pipe_count, &mut in_dir),
                    Pipe::EW => (),
                    _ => panic!("Invalid pipe"),
                }

                drawpipe(&mut frame, &map[y][x], x, y, 3);
            } else if pipe_count & 1 == 0 {
                // Even number of pipes crossed means this unvisited square is outide the loop
                // Draw in gray
                drawpipe(&mut frame, &map[y][x], x, y, 2);
            }
        }
    });

    // Draw final path frame
    draw_path(map, &mut gif, base_frame.clone(), range_start, &visited)?;

    gif.delay(1000)?;

    // Draw contained frame
    gif.draw_frame(frame, 1)?;

    gif.delay(1000)?;

    Ok(())
}

// Function to draw path
fn draw_path(
    map: &[MapRow],
    gif: &mut Gif,
    mut frame: Vec<Vec<u8>>,
    range_start: usize,
    visited: &[(usize, usize)],
) -> Result<(), Box<dyn Error>> {
    // Draw frame
    for (i, (px, py)) in visited.iter().enumerate() {
        let col = (i * RANGE_COLS) / max(1, visited.len() - 1);

        drawpipe(
            &mut frame,
            &map[*py][*px],
            *px,
            *py,
            (range_start + col) as u8,
        );
    }

    gif.draw_frame(frame, 1)?;

    Ok(())
}

// Function to draw pipe segment
fn drawpipe(frame: &mut [Vec<u8>], p: &Pipe, x: usize, y: usize, colour: u8) {
    // Get bitmap for pipe
    let bm = match p {
        Pipe::NS => [[0, 1, 0], [0, 1, 0], [0, 1, 0]],
        Pipe::EW => [[0, 0, 0], [1, 1, 1], [0, 0, 0]],
        Pipe::NE => [[0, 1, 0], [0, 1, 1], [0, 0, 0]],
        Pipe::NW => [[0, 1, 0], [1, 1, 0], [0, 0, 0]],
        Pipe::SW => [[0, 0, 0], [1, 1, 0], [0, 1, 0]],
        Pipe::SE => [[0, 0, 0], [0, 1, 1], [0, 1, 0]],
        Pipe::Ground => [[0, 0, 0], [0, 1, 0], [0, 0, 0]],
        _ => [[0, 0, 0], [0, 0, 0], [0, 0, 0]],
    };

    // Work out frame position
    let fy = y * CELLDIM;
    let fx = x * CELLDIM;

    // Draw the pixels
    for (yadd, r) in bm.iter().enumerate() {
        for (xadd, c) in r.iter().enumerate() {
            frame[fy + yadd][fx + xadd] = if *c == 1 { colour } else { 0 };
        }
    }
}

#[derive(Debug, PartialEq)]
enum Pipe {
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
    Ground,
    Start,
}

impl Pipe {
    fn next_dir(&self, dir: Dir) -> Dir {
        match self {
            Pipe::NS => match dir {
                Dir::S => Dir::S,
                Dir::N => Dir::N,
                _ => panic!("Invalid direction {dir:?} for {self:?}"),
            },
            Pipe::EW => match dir {
                Dir::E => Dir::E,
                Dir::W => Dir::W,
                _ => panic!("Invalid direction {dir:?} for {self:?}"),
            },
            Pipe::NE => match dir {
                Dir::S => Dir::E,
                Dir::W => Dir::N,
                _ => panic!("Invalid direction {dir:?} for {self:?}"),
            },
            Pipe::NW => match dir {
                Dir::S => Dir::W,
                Dir::E => Dir::N,
                _ => panic!("Invalid direction {dir:?} for {self:?}"),
            },
            Pipe::SW => match dir {
                Dir::E => Dir::S,
                Dir::N => Dir::W,
                _ => panic!("Invalid direction {dir:?} for {self:?}"),
            },
            Pipe::SE => match dir {
                Dir::N => Dir::E,
                Dir::W => Dir::S,
                _ => panic!("Invalid direction {dir:?} for {self:?}"),
            },
            _ => panic!("Invalid pipe"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Dir {
    N,
    E,
    S,
    W,
}

// Input parsing

type MapRow = Vec<Pipe>;

fn input_transform(line: String) -> MapRow {
    line.chars()
        .map(|c| match c {
            '|' => Pipe::NS,
            '-' => Pipe::EW,
            'L' => Pipe::NE,
            'J' => Pipe::NW,
            '7' => Pipe::SW,
            'F' => Pipe::SE,
            '.' => Pipe::Ground,
            'S' => Pipe::Start,
            _ => panic!("Invalid char"),
        })
        .collect()
}
