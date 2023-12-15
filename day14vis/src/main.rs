use std::{
    collections::HashMap,
    error::Error,
    hash::{DefaultHasher, Hash, Hasher},
};

use aoc::{
    gif::Gif,
    input::{parse_input_vec, parse_test_vec},
};

const SQUARE: usize = 6;

const EXAMPLE1: &str = "\
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

fn main() -> Result<(), Box<dyn Error>> {
    // Render example
    let map = parse_test_vec(EXAMPLE1, input_transform).unwrap();
    render(map, "vis/day14ex.gif", 4, Rate::Constant(1))?;

    // Get input and render
    let map = parse_input_vec(14, input_transform)?;
    render(map, "vis/day14.gif", 1, Rate::Fast(3))?;

    Ok(())
}

enum Rate {
    Fast(u8),
    Constant(u8),
}
fn render(
    mut map: Vec<InputEnt>,
    file: &str,
    scale: u16,
    rate: Rate,
) -> Result<(), Box<dyn Error>> {
    // Get input

    let mut hashes = HashMap::new();

    let mut i = 0;

    // Create GIF
    let palette: Vec<[u8; 3]> = vec![[0, 0, 0], [149, 141, 133], [83, 186, 183]];

    let mut gif = Gif::new(
        file,
        palette.as_slice(),
        (map[0].len() * SQUARE) as u16 + 1,
        (map.len() * SQUARE) as u16 + 1,
        scale,
        scale,
    )?;

    // Draw initail frame
    draw_map(&mut gif, &map)?;

    loop {
        // Roll the rocks
        roll(&mut gif, &mut map, Dir::N, &rate)?;
        roll(&mut gif, &mut map, Dir::W, &rate)?;
        roll(&mut gif, &mut map, Dir::S, &rate)?;
        roll(&mut gif, &mut map, Dir::E, &rate)?;

        // Hash the map
        let mut hasher = DefaultHasher::new();
        map.hash(&mut hasher);
        let new_hash = hasher.finish();

        // Already got this state?
        if hashes.contains_key(&new_hash) {
            break;
        }

        hashes.insert(new_hash, i);

        i += 1;
    }

    gif.delay(1000)?;

    Ok(())
}

type Move = ((usize, usize), (usize, usize), (isize, isize));

fn roll(gif: &mut Gif, map: &mut [InputEnt], dir: Dir, rate: &Rate) -> Result<(), Box<dyn Error>> {
    let mut moves = Vec::new();

    // Create base frame
    let mut frame = draw_frame(gif, map);

    match dir {
        Dir::N => (0..map[0].len()).for_each(|x| {
            (1..map.len()).for_each(|y| {
                if map[y][x] == State::Rock {
                    let lx = x;
                    roll_rock(
                        map,
                        x,
                        y,
                        (0..y).rev().map(move |ly| (lx, ly)),
                        rate,
                        &mut moves,
                    );
                }
            });
        }),
        Dir::S => (0..map[0].len()).for_each(|x| {
            (0..(map.len() - 1)).rev().for_each(|y| {
                if map[y][x] == State::Rock {
                    let lx = x;
                    roll_rock(
                        map,
                        x,
                        y,
                        ((y + 1)..map.len()).map(move |ly| (lx, ly)),
                        rate,
                        &mut moves,
                    );
                }
            });
        }),
        Dir::E => (0..map.len()).for_each(|y| {
            (0..(map[0].len() - 1)).rev().for_each(|x| {
                if map[y][x] == State::Rock {
                    let ly = y;
                    roll_rock(
                        map,
                        x,
                        y,
                        ((x + 1)..map[0].len()).map(move |lx| (lx, ly)),
                        rate,
                        &mut moves,
                    );
                }
            });
        }),
        Dir::W => (0..map.len()).for_each(|y| {
            (1..map[0].len()).for_each(|x| {
                if map[y][x] == State::Rock {
                    let ly = y;
                    roll_rock(
                        map,
                        x,
                        y,
                        (0..x).rev().map(move |lx| (lx, ly)),
                        rate,
                        &mut moves,
                    );
                }
            });
        }),
    };

    while !moves.is_empty() {
        // Blank out rocks
        for &((cx, cy), _, _) in moves.iter() {
            draw_cell(&mut frame, cx, cy, &State::Empty);
        }

        // Move rocks
        moves.iter_mut().for_each(|((cx, cy), _, (ix, iy))| {
            *cx = (*cx as isize + *ix) as usize;
            *cy = (*cy as isize + *iy) as usize;
        });

        // Draw rocks
        for &((cx, cy), _, _) in moves.iter() {
            draw_cell(&mut frame, cx, cy, &State::Rock);
        }

        gif.draw_frame(frame.clone(), 1)?;

        // Filter moves
        moves = moves
            .into_iter()
            .filter(|((cx, cy), (rx, ry), _)| cx != rx || cy != ry)
            .collect::<Vec<Move>>();
    }

    Ok(())
}

fn roll_rock(
    map: &mut [InputEnt],
    x: usize,
    y: usize,
    pos_iter: impl Iterator<Item = (usize, usize)>,
    rate: &Rate,
    moves: &mut Vec<Move>,
) {
    map[y][x] = State::Empty;

    let (mut rx, mut ry) = (x, y);

    for (cx, cy) in pos_iter {
        if map[cy][cx] == State::Empty {
            (rx, ry) = (cx, cy)
        } else {
            break;
        }
    }

    map[ry][rx] = State::Rock;

    if rx != x || ry != y {
        let (ix, iy) = match rate {
            Rate::Fast(rate) => {
                let ix = (rx as isize - x as isize) * (SQUARE as isize / *rate as isize);
                let iy = (ry as isize - y as isize) * (SQUARE as isize / *rate as isize);
                (ix, iy)
            }
            Rate::Constant(rate) => {
                let ix = (rx as isize - x as isize).signum() * *rate as isize;
                let iy = (ry as isize - y as isize).signum() * *rate as isize;
                (ix, iy)
            }
        };

        moves.push((
            (x * SQUARE, y * SQUARE),
            (rx * SQUARE, ry * SQUARE),
            (ix, iy),
        ));
    }
}

fn draw_map(gif: &mut Gif, map: &[InputEnt]) -> Result<(), Box<dyn Error>> {
    let frame = draw_frame(gif, map);

    gif.draw_frame(frame, 5)?;

    Ok(())
}

fn draw_frame(gif: &Gif, map: &[InputEnt]) -> Vec<Vec<u8>> {
    let mut frame = gif.empty_frame();

    let mut gy = 0;

    for line in map {
        let mut gx = 0;

        for cell in line {
            draw_cell(&mut frame, gx, gy, cell);

            gx += SQUARE;
        }

        gy += SQUARE;
    }

    frame
}

fn draw_cell(frame: &mut Vec<Vec<u8>>, gx: usize, gy: usize, cell: &State) {
    let draw_pixels = |frame: &mut Vec<Vec<u8>>,
                       gx: usize,
                       gy: usize,
                       bmp: &[[_; SQUARE]; SQUARE],
                       colour: u8| {
        for y in 0..SQUARE {
            for x in 0..SQUARE {
                if bmp[y][x] == 1 {
                    frame[gy + y][gx + x] = colour;
                }
            }
        }
    };

    match cell {
        State::Empty => {
            draw_pixels(
                frame,
                gx,
                gy,
                &[
                    [1, 1, 1, 1, 1, 1],
                    [1, 1, 1, 1, 1, 1],
                    [1, 1, 1, 1, 1, 1],
                    [1, 1, 1, 1, 1, 1],
                    [1, 1, 1, 1, 1, 1],
                    [1, 1, 1, 1, 1, 1],
                ],
                0,
            );
        }
        State::Rock => {
            draw_pixels(
                frame,
                gx,
                gy,
                &[
                    [0, 0, 0, 0, 0, 0],
                    [0, 0, 1, 1, 1, 0],
                    [0, 1, 1, 1, 1, 1],
                    [0, 1, 1, 1, 1, 1],
                    [0, 1, 1, 1, 1, 1],
                    [0, 0, 1, 1, 1, 0],
                ],
                1,
            );
        }
        State::Cube => {
            draw_pixels(
                frame,
                gx,
                gy,
                &[
                    [0, 0, 0, 0, 0, 0],
                    [0, 1, 1, 1, 1, 1],
                    [0, 1, 1, 1, 1, 1],
                    [0, 1, 1, 1, 1, 1],
                    [0, 1, 1, 1, 1, 1],
                    [0, 1, 1, 1, 1, 1],
                ],
                2,
            );
        }
    }
}

#[derive(Debug, PartialEq, Clone, Hash)]
enum State {
    Empty,
    Rock,
    Cube,
}

#[derive(Debug)]
enum Dir {
    N,
    E,
    S,
    W,
}

// Input parsing

type InputEnt = Vec<State>;

fn input_transform(line: String) -> InputEnt {
    line.chars()
        .map(|c| match c {
            '.' => State::Empty,
            '#' => State::Cube,
            'O' => State::Rock,
            _ => panic!("Invalid char"),
        })
        .collect::<Vec<_>>()
}
