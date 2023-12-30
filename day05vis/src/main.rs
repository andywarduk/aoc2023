use std::{collections::VecDeque, error::Error, fs::File, io::Write, ops::Range};

use aoc::input::{parse_input_vec, parse_test_vec};

fn main() -> Result<(), Box<dyn Error>> {
    let input = parse_test_vec(EXAMPLE1, input_transform).unwrap();
    let almanac = parse_lines(&input);
    part2(&almanac, "vis/day05ex.html", 500)?;

    let input = parse_input_vec(5, input_transform)?;
    let almanac = parse_lines(&input);
    part2(&almanac, "vis/day05-2.html", 1500)?;

    Ok(())
}

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

/// Work queue entry
struct RangeItem {
    depth: usize,
    range: Range<u64>,
}

fn part2(almanac: &Almanac, output: &str, height: usize) -> Result<(), Box<dyn Error>> {
    let mut nodes = Vec::new();
    let mut edges = Vec::new();

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
    while let Some(RangeItem { depth, range }) = range_queue.pop_front() {
        // Add node
        if nodes.len() <= depth {
            nodes.push(vec![]);
        }

        nodes[depth].push(Node {
            range: range.clone(),
        });

        if depth >= almanac.maps.len() {
            continue;
        }

        let mut range_remain = range.clone();

        let mut next = |depth: usize, from: Range<u64>, to: Range<u64>| {
            println!(
                "{depth}: {}-{} -> {}-{}",
                from.start, from.end, to.start, to.end
            );

            // Add edge
            edges.push(Edge {
                depth,
                src: from,
                dst: to.clone(),
            });

            // Add mapped range to the queue
            range_queue.push_back(RangeItem {
                depth: depth + 1,
                range: to,
            });
        };

        for item_map in &almanac.maps[depth] {
            let ItemMap { dest: _, source } = item_map;

            // Find range overlap
            if range_remain.start >= source.start && range_remain.end <= source.end {
                // Completely contained
                let mapped_range = item_map.map(&range_remain);

                next(depth, range.clone(), mapped_range.clone());

                // Make range empty
                range_remain = Range { start: 1, end: 0 };

                break;
            } else if range_remain.start >= source.start && range_remain.start <= source.end {
                // Start overlaps
                // range        |---------|
                // source  |----------|

                // Calculate overlap length
                let overlap_len = source.end - range_remain.start;

                // Calculate overlap range
                let overlap_range = Range {
                    start: range_remain.start,
                    end: range_remain.start + overlap_len,
                };

                // Map the overlap range
                let mapped_range = item_map.map(&overlap_range);

                // Calculate left over range
                let new_range = Range {
                    start: range_remain.start + overlap_len + 1,
                    end: range_remain.end,
                };

                next(depth, range.clone(), mapped_range.clone());

                // Set new work range
                range_remain = new_range;
                println!("  remainder {}-{}", range_remain.start, range_remain.end);
            } else if range_remain.end >= source.start && range_remain.end <= source.end {
                // End overlaps
                // range   |---------|
                // source       |----------|

                // Calculate overlap length
                let overlap_len = range_remain.end - source.start;

                // Calculate overlap range
                let overlap_range = Range {
                    start: source.start,
                    end: source.start + overlap_len,
                };

                // Map the overlap range
                let mapped_range = item_map.map(&overlap_range);

                // Calculate left over range
                let new_range = Range {
                    start: range_remain.start,
                    end: source.start - 1,
                };

                next(depth, range.clone(), mapped_range.clone());

                // Set new work range
                range_remain = new_range;
                println!("  remainder {}-{}", range_remain.start, range_remain.end);
            }
        }

        if !range_remain.is_empty() {
            next(depth, range.clone(), range_remain);
        }
    }

    edges.sort();

    write_doc(nodes, edges, output, height)?;

    Ok(())
}

fn write_doc(
    nodes: Vec<Vec<Node>>,
    edges: Vec<Edge>,
    output: &str,
    height: usize,
) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(output)?;

    file.write_all(
        r#"<!DOCTYPE html>
<html>

<head>
    <script src="https://code.highcharts.com/highcharts.js"></script>
    <script src="https://code.highcharts.com/modules/sankey.js"></script>
    <script src="https://code.highcharts.com/modules/exporting.js"></script>
    <style>
        #container {
            height: 2000px;
        }
"#
        .as_bytes(),
    )?;

    file.write_fmt(format_args!(
        "        #container {{
            height: {height}px;
        }}",
    ))?;

    file.write_all(
        r#"    </style>
</head>

<body>
    <div id="container"></div>

    <script type="text/javascript">
        Highcharts.chart('container', {
            chart: {
                inverted: false
            },
            title: {
                text: ''
            },
            accessibility: {
                point: {
                    valueDescriptionFormat: '{index}. {point.from} to {point.to}, {point.weight}.'
                }
            },
            tooltip: {
                headerFormat: null,
                pointFormat:
                    '{point.fromNode.name} \u2192 {point.toNode.name}: ({point.weight})',
                nodeFormat: '{point.name} ({point.sum})'
            },
            series: [{
                keys: ['from', 'to', 'weight'],

                dataLabels: {
                    enabled: false,
                },

                nodes: [
"#
        .as_bytes(),
    )?;

    for (depth, nv) in nodes.iter().enumerate() {
        let mut nv = nv.to_vec();

        nv.sort();

        for n in nv {
            file.write_fmt(format_args!(
                "                    {{
                        id: '{}-{}-{}',
                        column: {},
                    }},
",
                depth, n.range.start, n.range.end, depth
            ))?;
        }
    }

    file.write_all(
        r#"                ],

                data: [
"#
        .as_bytes(),
    )?;

    for e in edges {
        file.write_fmt(format_args!(
            "                    ['{}-{}-{}', '{}-{}-{}', {}],\n",
            e.depth,
            e.src.start,
            e.src.end,
            e.depth + 1,
            e.dst.start,
            e.dst.end,
            (e.dst.end - e.dst.start) + 1
        ))?;
    }

    file.write_all(
        r#"                ],
                type: 'sankey',
                name: 'Seed mixer'
            }]

        });
        </script>

    </body>

</html>
"#
        .as_bytes(),
    )?;

    Ok(())
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

#[derive(Debug, Clone, PartialEq, Eq)]
struct Node {
    range: Range<u64>,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.range
            .start
            .cmp(&other.range.start)
            .then(self.range.end.cmp(&other.range.end))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Edge {
    depth: usize,
    src: Range<u64>,
    dst: Range<u64>,
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.depth
            .cmp(&other.depth)
            .then(self.dst.start.cmp(&other.src.start))
            .then(self.dst.end.cmp(&other.src.end))
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
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
