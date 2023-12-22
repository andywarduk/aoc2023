use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet},
    error::Error,
};

use aoc::input::parse_input_vec;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let mut bricks = parse_input_vec(22, input_transform)?;

    let map = compress(&mut bricks);
    let (supports, supported_by) = supports(&bricks, &map);

    // Run parts
    println!("Part 1: {}", part1(&supports, &supported_by));
    println!("Part 2: {}", part2(&supports, &supported_by));

    Ok(())
}

fn part1(supports: &[HashSet<usize>], supported_by: &[HashSet<usize>]) -> u64 {
    // Count bricks which can be removed
    supports.iter().enumerate().fold(0, |count, (i, supports)| {
        for s in supports {
            if !supported_by[*s].iter().any(|s| *s != i) {
                return count;
            }
        }

        count + 1
    })
}

fn part2(supports: &[HashSet<usize>], supported_by: &[HashSet<usize>]) -> u64 {
    let mut count = 0;

    for i in 0..supports.len() {
        // Start fallers hash set
        let mut fallers = HashSet::new();
        fallers.insert(i);

        // Recursively check supported bricks
        p2rec(i, supports, supported_by, &mut fallers);

        count += fallers.len() - 1;
    }

    count as u64
}

fn p2rec(
    i: usize,
    supports: &[HashSet<usize>],
    supported_by: &[HashSet<usize>],
    fallers: &mut HashSet<usize>,
) {
    // Loop supported bricks
    for s in supports[i].iter() {
        // Are the bricks supporting this brick all falling?
        if supported_by[*s].is_subset(fallers) {
            // Yes - this one is falling too
            fallers.insert(*s);

            // Recurse
            p2rec(*s, supports, supported_by, fallers);
        }
    }
}

fn compress(bricks: &mut [Brick]) -> HashMap<(u16, u16, u16), usize> {
    // Collect minimum zs
    let mut zs = bricks
        .iter()
        .enumerate()
        .map(|(i, b)| (min(b.pos[0].z, b.pos[1].z), i))
        .collect::<Vec<_>>();

    // Sort zs
    zs.sort();

    let mut occupied = HashMap::new();

    for (sz, i) in zs {
        let mut brick = bricks[i].clone();

        let mut new_z = None;

        for tryz in (1..sz).rev() {
            // Move the brick
            brick.pos[0].z = bricks[i].pos[0].z - sz + tryz;
            brick.pos[1].z = bricks[i].pos[1].z - sz + tryz;

            // Does it clash?
            let positions = brick.positions();

            let mut clash = false;

            for p in positions {
                if occupied.contains_key(&p) {
                    clash = true;
                    break;
                }
            }

            if clash {
                break;
            } else {
                new_z = Some(tryz);
            }
        }

        // Move the brick
        if let Some(new_z) = new_z {
            bricks[i].pos[0].z = bricks[i].pos[0].z - sz + new_z;
            bricks[i].pos[1].z = bricks[i].pos[1].z - sz + new_z;
        }

        // Set occupied
        for p in bricks[i].positions() {
            let inserted = occupied.insert(p, i);
            debug_assert!(inserted.is_none());
        }
    }

    occupied
}

fn supports(
    bricks: &[Brick],
    map: &HashMap<(u16, u16, u16), usize>,
) -> (Vec<HashSet<usize>>, Vec<HashSet<usize>>) {
    // Find out what supports what
    let mut supports = vec![HashSet::new(); bricks.len()];
    let mut supported_by = vec![HashSet::new(); bricks.len()];

    for (i, brick) in bricks.iter().enumerate() {
        let positions = brick.positions();

        // What bricks is this brick supporting?
        for (x, y, z) in positions {
            let above = (x, y, z + 1);

            if let Some(ent) = map.get(&above) {
                if *ent != i {
                    supports[i].insert(*ent);
                    supported_by[*ent].insert(i);
                }
            }
        }
    }

    (supports, supported_by)
}

#[derive(Debug, Clone, Copy)]
struct Pos {
    x: u16,
    y: u16,
    z: u16,
}

#[derive(Debug, Clone)]
struct Brick {
    pos: [Pos; 2],
}

impl Brick {
    fn positions(&self) -> Vec<(u16, u16, u16)> {
        let x1 = min(self.pos[0].x, self.pos[1].x);
        let x2 = max(self.pos[0].x, self.pos[1].x);
        let y1 = min(self.pos[0].y, self.pos[1].y);
        let y2 = max(self.pos[0].y, self.pos[1].y);
        let z1 = min(self.pos[0].z, self.pos[1].z);
        let z2 = max(self.pos[0].z, self.pos[1].z);

        let mut positions = Vec::new();

        (x1..=x2).for_each(|x| {
            (y1..=y2).for_each(|y| (z1..=z2).for_each(|z| positions.push((x, y, z))))
        });

        positions
    }
}
// Input parsing

fn input_transform(line: String) -> Brick {
    let pos = line
        .split('~')
        .map(|coords| {
            let coords = coords
                .split(',')
                .map(|c| c.parse::<u16>().unwrap())
                .collect::<Vec<_>>();

            Pos {
                x: coords[0],
                y: coords[1],
                z: coords[2],
            }
        })
        .collect::<Vec<_>>();

    Brick {
        pos: [pos[0], pos[1]],
    }
}

#[cfg(test)]
mod tests {
    use aoc::input::parse_test_vec;

    use super::*;

    const EXAMPLE1: &str = "\
1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";

    #[test]
    fn test1() {
        let mut bricks = parse_test_vec(EXAMPLE1, input_transform).unwrap();
        let map = compress(&mut bricks);
        let (supports, supported_by) = supports(&bricks, &map);

        assert_eq!(part1(&supports, &supported_by), 5);
        assert_eq!(part2(&supports, &supported_by), 7);
    }
}
