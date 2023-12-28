use std::{
    collections::{hash_map::Entry, HashMap, HashSet, VecDeque},
    error::Error,
    fs::File,
    io::Write,
};

use aoc::{
    gif::{Gif, IdenticalAction},
    input::parse_input_vec,
};

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(23, input_transform)?;

    part1(&input, "vis/day23-1.gif")?;
    part2(&input, "vis/day23-2.gif", "vis/day23-2.dot")?;

    Ok(())
}

fn part1(map: &[MapRow], file: &str) -> Result<(), Box<dyn Error>> {
    let mut longest = vec![];

    let (sx, sy, ex, ey) = find_exits(map);

    let mut queue = VecDeque::new();
    let mut visited = HashMap::new();

    queue.push_back((vec![(sx, sy)], Dir::S, 0));

    while let Some((path, dir, steps)) = queue.pop_back() {
        let (x, y) = path[path.len() - 1];

        if x == ex && y == ey {
            if path.len() > longest.len() {
                longest = path;
            }
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

            let mut new_path = path.clone();
            new_path.push((x, y));

            queue.push_back((new_path, dir, steps));
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

    // Create GIF
    create_gif(map, file, &longest, None)
}

type Edges = HashMap<Node, Vec<(Node, Vec<(usize, usize)>)>>;

fn part2(map: &[MapRow], gif_file: &str, dot_file: &str) -> Result<(), Box<dyn Error>> {
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
    let mut edges: Edges = HashMap::new();

    for (x, y) in nodes.iter() {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();

        queue.push_back((*x, *y, vec![], *x, *y));
        visited.insert((*x, *y));

        while let Some((x, y, steps, nx, ny)) = queue.pop_front() {
            if (x != nx || y != ny) && nodes.contains(&(x, y)) {
                // On a node - add edge
                let ent = ((x, y), steps);

                edges
                    .entry((nx, ny))
                    .and_modify(|e| e.push(ent.clone()))
                    .or_insert(vec![ent]);

                continue;
            }

            let mut move_to = |x: usize, y: usize| {
                if !matches!(map[y][x], Tile::Forest) && !visited.contains(&(x, y)) {
                    let mut new_steps = steps.clone();
                    new_steps.push((x, y));

                    queue.push_back((x, y, new_steps, nx, ny));
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
        nodes: vec![(sx, sy)],
        visited: HashSet::new(),
    };

    let (_, node_list) = find_longest(state, ex, ey, &edges);

    // Recreate path
    let mut longest = vec![(sx, sy)];

    longest.extend(
        node_list
            .iter()
            .skip(1)
            .fold(
                ((sx, sy), vec![]),
                |((lx, ly), mut path): ((usize, usize), Vec<(usize, usize)>), &(nx, ny)| {
                    let steps = edges
                        .get(&(lx, ly))
                        .unwrap()
                        .iter()
                        .find_map(|((x, y), steps)| {
                            if *x == nx && *y == ny {
                                Some(steps)
                            } else {
                                None
                            }
                        })
                        .unwrap();

                    path.extend(steps);

                    ((nx, ny), path)
                },
            )
            .1,
    );

    // Create dot file
    write_dot(dot_file, &nodes, &edges, &node_list)?;

    // Create GIF
    create_gif(map, gif_file, &longest, Some(nodes))?;

    Ok(())
}

fn write_dot(
    file: &str,
    nodes: &HashSet<(usize, usize)>,
    edges: &Edges,
    node_list: &[(usize, usize)],
) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(file).unwrap();

    file.write_fmt(format_args!("digraph {{\nrankdir=\"LR\";\n"))?;

    // Write nodes
    let mut nodes_vec = nodes.iter().collect::<Vec<_>>();

    nodes_vec.sort_by(|(ax, ay), (bx, by)| {
        (ax * ay)
            .cmp(&(bx * by))
            .then_with(|| ay.cmp(by))
            .then_with(|| ax.cmp(bx))
    });

    for (x, y) in &nodes_vec {
        file.write_fmt(format_args!("\"{x}x{y}\" [shape=box"))?;

        if node_list.contains(&(*x, *y)) {
            file.write_all(b" color=red")?;
        }

        file.write_all(b"];\n")?;
    }

    // Write edges
    let mut done = HashSet::new();

    for (x1, y1) in nodes_vec {
        done.insert((x1, y1));

        let edges = edges.get(&(*x1, *y1)).unwrap();

        let pos1 = node_list.iter().position(|(x, y)| x == x1 && y == y1);

        for ((x2, y2), steps) in edges {
            if !done.contains(&(x2, y2)) {
                let mut arrow_head = "none";
                let mut arrow_tail = "none";
                let mut on_path = false;

                let pos2 = node_list.iter().position(|(x, y)| x == x2 && y == y2);

                if let Some(pos1) = pos1 {
                    if let Some(pos2) = pos2 {
                        match pos1 as isize - pos2 as isize {
                            -1 => {
                                arrow_head = "normal";
                                on_path = true;
                            }
                            1 => {
                                arrow_tail = "normal";
                                on_path = true;
                            }
                            _ => (),
                        }
                    }
                }

                file.write_fmt(format_args!(
                    "\"{x1}x{y1}\" -> \"{x2}x{y2}\" [label={} dir=both",
                    steps.len()
                ))?;

                if on_path {
                    file.write_all(b" color=red")?;
                }

                file.write_fmt(format_args!(" arrowhead={arrow_head}"))?;
                file.write_fmt(format_args!(" arrowtail={arrow_tail}"))?;

                file.write_all(b"];\n")?;
            }
        }
    }

    file.write_fmt(format_args!("}}\n"))?;

    Ok(())
}

type Node = (usize, usize);

struct State {
    x: usize,
    y: usize,
    steps: u64,
    nodes: Vec<(usize, usize)>,
    visited: HashSet<Node>,
}

fn find_longest(
    mut state: State,
    ex: usize,
    ey: usize,
    edges: &Edges,
) -> (u64, Vec<(usize, usize)>) {
    state.visited.insert((state.x, state.y));

    edges
        .get(&(state.x, state.y))
        .unwrap()
        .iter()
        .filter_map(|((x, y), steps)| {
            if state.visited.contains(&(*x, *y)) {
                None
            } else {
                let steps = state.steps + steps.len() as u64;
                let mut new_nodes = state.nodes.clone();
                new_nodes.push((*x, *y));

                Some(if *x == ex && *y == ey {
                    (steps, new_nodes)
                } else {
                    find_longest(
                        State {
                            x: *x,
                            y: *y,
                            steps,
                            nodes: new_nodes,
                            visited: state.visited.clone(),
                        },
                        ex,
                        ey,
                        edges,
                    )
                })
            }
        })
        .fold((0, vec![]), |(msteps, mnodes), (steps, nodes)| {
            if steps > msteps {
                (steps, nodes)
            } else {
                (msteps, mnodes)
            }
        })
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

fn create_gif(
    map: &[MapRow],
    file: &str,
    path: &[(usize, usize)],
    nodes: Option<HashSet<(usize, usize)>>,
) -> Result<(), Box<dyn Error>> {
    // Create GIF
    let palette: [[u8; 3]; 4] = [[0, 0, 0], [64, 255, 64], [255, 255, 64], [128, 64, 255]];

    let mut gif = Gif::new(
        file,
        &palette,
        map[0].len() as u16 * 5,
        map.len() as u16 * 5,
        1,
        1,
    )?;

    // Draw base frame
    let mut frame = base_frame(map, &gif, nodes);

    gif.draw_frame(frame.clone(), 0)?;

    // Animate path
    for (i, (x, y)) in path.iter().enumerate() {
        let gx = x * 5;
        let gy = y * 5;

        for y in 0..5 {
            for x in 0..5 {
                if frame[gy + y][gx + x] == 0 {
                    frame[gy + y][gx + x] = 3;
                }
            }
        }

        if i % 5 == 0 {
            gif.draw_frame(frame.clone(), 1)?;
        }
    }

    gif.draw_frame_identical_check(frame, 1000, IdenticalAction::Delay)?;

    Ok(())
}

fn base_frame(map: &[MapRow], gif: &Gif, nodes: Option<HashSet<(usize, usize)>>) -> Vec<Vec<u8>> {
    let mut frame = gif.empty_frame();

    (0..map.len()).for_each(|y| {
        let gy = y * 5;

        for x in 0..map[y].len() {
            let gx = x * 5;

            let bmp = if matches!(map[y][x], Tile::Forest) {
                [
                    [0, 1, 1, 1, 0],
                    [1, 1, 1, 1, 1],
                    [1, 1, 1, 1, 1],
                    [1, 1, 1, 1, 1],
                    [0, 1, 1, 1, 0],
                ]
            } else if let Some(nodes) = &nodes {
                if nodes.contains(&(x, y)) {
                    [
                        [0, 0, 0, 0, 0],
                        [0, 2, 2, 2, 0],
                        [0, 2, 2, 2, 0],
                        [0, 2, 2, 2, 0],
                        [0, 0, 0, 0, 0],
                    ]
                } else {
                    [
                        [0, 0, 0, 0, 0],
                        [0, 0, 0, 0, 0],
                        [0, 0, 0, 0, 0],
                        [0, 0, 0, 0, 0],
                        [0, 0, 0, 0, 0],
                    ]
                }
            } else {
                match map[y][x] {
                    Tile::Path => [
                        [0, 0, 0, 0, 0],
                        [0, 0, 0, 0, 0],
                        [0, 0, 0, 0, 0],
                        [0, 0, 0, 0, 0],
                        [0, 0, 0, 0, 0],
                    ],
                    Tile::SlopeN => [
                        [0, 0, 0, 0, 0],
                        [0, 0, 2, 0, 0],
                        [0, 2, 2, 2, 0],
                        [2, 2, 2, 2, 2],
                        [0, 0, 0, 0, 0],
                    ],
                    Tile::SlopeW => [
                        [0, 0, 0, 2, 0],
                        [0, 0, 2, 2, 0],
                        [0, 2, 2, 2, 0],
                        [0, 0, 2, 2, 0],
                        [0, 0, 0, 2, 0],
                    ],
                    Tile::SlopeE => [
                        [0, 2, 0, 0, 0],
                        [0, 2, 2, 0, 0],
                        [0, 2, 2, 2, 0],
                        [0, 2, 2, 0, 0],
                        [0, 2, 0, 0, 0],
                    ],
                    Tile::SlopeS => [
                        [0, 0, 0, 0, 0],
                        [2, 2, 2, 2, 2],
                        [0, 2, 2, 2, 0],
                        [0, 0, 2, 0, 0],
                        [0, 0, 0, 0, 0],
                    ],
                    _ => unreachable!(),
                }
            };

            for (y, br) in bmp.iter().enumerate() {
                for (x, bc) in br.iter().enumerate() {
                    if *bc != 0 {
                        frame[gy + y][gx + x] = *bc;
                    }
                }
            }
        }
    });

    frame
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
