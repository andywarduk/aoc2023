use std::{cmp::min, collections::VecDeque, error::Error, ops::Range};

use aoc::input::parse_input_vec;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(5, input_transform)?;
    let almanac = parse_lines(&input);

    // Run parts
    println!("Part 1: {}", part1(&almanac));
    println!("Part 2: {}", part2(&almanac));

    Ok(())
}

fn part1(almanac: &Almanac) -> u64 {
    let mut result = None;

    // Iterate each seed
    for s in &almanac.seeds {
        let mut num = *s;

        // Iterate each map
        for m in &almanac.maps {
            // Iterate each map element
            for e in m {
                // Seed contained in this range?
                if e.source.start <= num && e.source.end >= num {
                    // Translate the number
                    num = e.dest.start + (num - e.source.start);

                    break;
                }
            }
        }

        // Update result
        result = match result {
            Some(r) => Some(min(r, num)),
            None => Some(num),
        }
    }

    result.unwrap_or(0)
}

/// Work queue entry
struct RangeItem {
    depth: usize,
    range: Range<u64>,
}

fn part2(almanac: &Almanac) -> u64 {
    let mut result = None;

    // Add seed ranges to the work queue
    let mut range_queue: VecDeque<RangeItem> = almanac
        .seeds
        .chunks_exact(2)
        .map(|c| RangeItem {
            depth: 0,
            range: Range {
                start: c[0],
                end: (c[0] + c[1]) - 1,
            },
        })
        .collect();

    // Process each work queue item
    while let Some(RangeItem { depth, mut range }) = range_queue.pop_front() {
        #[cfg(test)]
        println!("-- {range:?} {depth} --");

        if depth >= almanac.maps.len() {
            // No more maps to work on - update result with lower bound of the range
            result = match result {
                Some(result) => Some(min(result, range.start)),
                None => Some(range.start),
            };

            continue;
        }

        for item_map in &almanac.maps[depth] {
            #[cfg(test)]
            let ItemMap { dest, source } = item_map;
            #[cfg(not(test))]
            let ItemMap { dest: _, source } = item_map;

            // Find range overlap
            if range.start >= source.start && range.end <= source.end {
                // Completely contained
                let mapped_range = item_map.map(&range);

                #[cfg(test)]
                println!(
                    "{:?} (depth {depth}) contained in {:?} ({:?}) -> {:?}",
                    range, source, dest, mapped_range
                );

                // Add mapped range to the queue
                range_queue.push_back(RangeItem {
                    depth: depth + 1,
                    range: mapped_range,
                });

                // Make range empty
                range = Range { start: 1, end: 0 };

                break;
            } else if range.start >= source.start && range.start <= source.end {
                // Start overlaps
                // range        |---------|
                // source  |----------|

                // Calculate overlap length
                let overlap_len = source.end - range.start;

                // Calculate overlap range
                let overlap_range = Range {
                    start: range.start,
                    end: range.start + overlap_len,
                };

                // Map the overlap range
                let mapped_range = item_map.map(&overlap_range);

                // Calculate left over range
                let new_range = Range {
                    start: range.start + overlap_len + 1,
                    end: range.end,
                };

                #[cfg(test)]
                println!(
                    "{:?} (depth {depth}) overlaps {:?} at start -> {:?} ({:?}), {:?}",
                    range, source, overlap_range, mapped_range, new_range
                );

                // Add mapped range to the queue
                range_queue.push_back(RangeItem {
                    depth: depth + 1,
                    range: mapped_range,
                });

                // Set new work range
                range = new_range;
            } else if range.end >= source.start && range.end <= source.end {
                // End overlaps
                // range   |---------|
                // source       |----------|

                // Calculate overlap length
                let overlap_len = range.end - source.start;

                // Calculate overlap range
                let overlap_range = Range {
                    start: source.start,
                    end: source.start + overlap_len,
                };

                // Map the overlap range
                let mapped_range = item_map.map(&overlap_range);

                // Calculate left over range
                let new_range = Range {
                    start: range.start,
                    end: source.start - 1,
                };

                #[cfg(test)]
                println!(
                    "{:?} (depth {depth}) overlaps {:?} at end -> {:?} ({:?}), {:?}",
                    range, source, overlap_range, mapped_range, new_range
                );

                // Add mapped range to the queue
                range_queue.push_back(RangeItem {
                    depth: depth + 1,
                    range: mapped_range,
                });

                // Set new work range
                range = new_range;
            }
        }

        if !range.is_empty() {
            #[cfg(test)]
            println!("Adding {range:?} (depth {})", depth + 1);

            range_queue.push_back(RangeItem {
                depth: depth + 1,
                range,
            });
        }
    }

    result.unwrap_or(0)
}

#[derive(Default)]
struct Almanac {
    seeds: Vec<u64>,
    maps: Vec<Vec<ItemMap>>,
}

struct ItemMap {
    dest: Range<u64>,
    source: Range<u64>,
}

impl ItemMap {
    fn new(line: &str) -> Self {
        let mut nums = line
            .split_ascii_whitespace()
            .map(|n| n.parse::<u64>().unwrap());

        let dest_start = nums.next().unwrap();
        let source_start = nums.next().unwrap();
        let length = nums.next().unwrap();

        ItemMap {
            dest: dest_start..(dest_start + length - 1),
            source: source_start..(source_start + length - 1),
        }
    }

    fn map(&self, range: &Range<u64>) -> Range<u64> {
        let dest_start = self.dest.start + (range.start - self.source.start);

        Range {
            start: dest_start,
            end: dest_start + (range.end - range.start),
        }
    }
}

// Input parsing

type InputEnt = String;

fn input_transform(line: String) -> InputEnt {
    line
}

fn parse_lines(lines: &[String]) -> Almanac {
    let mut almanac = Almanac::default();
    let mut cur_vec: Option<Vec<ItemMap>> = None;

    for l in lines {
        if l.starts_with("seeds:") {
            almanac.seeds = l
                .split_ascii_whitespace()
                .skip(1)
                .map(|n| n.parse::<u64>().unwrap())
                .collect();
        } else if l.is_empty() {
            if let Some(vec) = cur_vec {
                if !vec.is_empty() {
                    almanac.maps.push(vec);
                }
            }

            cur_vec = None;
        } else if let Some(vec) = &mut cur_vec {
            vec.push(ItemMap::new(l))
        } else {
            cur_vec = Some(Vec::new());
        }
    }

    if let Some(vec) = cur_vec {
        if !vec.is_empty() {
            almanac.maps.push(vec);
        }
    }

    almanac
}

#[cfg(test)]
mod tests {
    use aoc::input::parse_test_vec;

    use super::*;

    const EXAMPLE1: &str = "\
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn test1() {
        let input = parse_test_vec(EXAMPLE1, input_transform).unwrap();
        let almanac = parse_lines(&input);

        assert_eq!(part1(&almanac), 35);
        assert_eq!(part2(&almanac), 46);
    }
}
