use std::{
    collections::HashMap,
    error::Error,
    hash::{DefaultHasher, Hash, Hasher},
};

use aoc::{gif::Gif, input::parse_input_vec};

const SQUARE: usize = 6;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let mut map = parse_input_vec(14, input_transform)?;

    let mut hashes = HashMap::new();

    let mut i = 0;

    // Create GIF
    let palette: Vec<[u8; 3]> = vec![[0, 0, 0], [149, 141, 133], [83, 186, 183]];
    let gif_width = (map[0].len() * SQUARE) as u16 + 1;
    let gif_height = (map.len() * SQUARE) as u16 + 1;

    let mut gif = Gif::new(
        "vis/day14.gif",
        palette.as_slice(),
        gif_width,
        gif_height,
        1,
        1,
    )?;

    draw_map(&mut gif, gif_width, gif_height, &map)?;

    loop {
        // Roll the rocks
        roll(&mut map, Dir::N);
        draw_map(&mut gif, gif_width, gif_height, &map)?;
        roll(&mut map, Dir::W);
        draw_map(&mut gif, gif_width, gif_height, &map)?;
        roll(&mut map, Dir::S);
        draw_map(&mut gif, gif_width, gif_height, &map)?;
        roll(&mut map, Dir::E);
        draw_map(&mut gif, gif_width, gif_height, &map)?;

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

fn roll(map: &mut [InputEnt], dir: Dir) {
    match dir {
        Dir::N => (0..map[0].len()).for_each(|x| {
            (1..map.len()).for_each(|y| {
                if map[y][x] == State::Rock {
                    let lx = x;
                    roll_rock(map, x, y, (0..y).rev().map(move |ly| (lx, ly)));
                }
            });
        }),
        Dir::S => (0..map[0].len()).for_each(|x| {
            (0..(map.len() - 1)).rev().for_each(|y| {
                if map[y][x] == State::Rock {
                    let lx = x;
                    roll_rock(map, x, y, ((y + 1)..map.len()).map(move |ly| (lx, ly)));
                }
            });
        }),
        Dir::E => (0..map.len()).for_each(|y| {
            (0..(map[0].len() - 1)).rev().for_each(|x| {
                if map[y][x] == State::Rock {
                    let ly = y;
                    roll_rock(map, x, y, ((x + 1)..map[0].len()).map(move |lx| (lx, ly)));
                }
            });
        }),
        Dir::W => (0..map.len()).for_each(|y| {
            (1..map[0].len()).for_each(|x| {
                if map[y][x] == State::Rock {
                    let ly = y;
                    roll_rock(map, x, y, (0..x).rev().map(move |lx| (lx, ly)));
                }
            });
        }),
    }
}

fn roll_rock(
    map: &mut [InputEnt],
    x: usize,
    y: usize,
    pos_iter: impl Iterator<Item = (usize, usize)>,
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
}

fn draw_map(
    gif: &mut Gif,
    width: u16,
    height: u16,
    map: &[InputEnt],
) -> Result<(), Box<dyn Error>> {
    let mut frame = vec![vec![0; width as usize]; height as usize];

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

    let mut gy = 0;

    for line in map {
        let mut gx = 0;

        for cell in line {
            match cell {
                State::Empty => (),
                State::Rock => {
                    draw_pixels(
                        &mut frame,
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
                        &mut frame,
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

            gx += SQUARE;
        }

        gy += SQUARE;
    }

    gif.draw_frame(frame, 5)?;

    Ok(())
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
