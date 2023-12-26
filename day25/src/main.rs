use std::{collections::HashMap, error::Error};

use petgraph::graph::UnGraph;
use rustworkx_core::connectivity::stoer_wagner_min_cut;

use aoc::input::parse_input_vec;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(25, input_transform)?;

    // Run parts
    println!("Part 1: {}", part1(&input));

    Ok(())
}

fn part1(input: &[Conn]) -> u64 {
    let nodes = get_nodes(input);

    let mut graph = UnGraph::<String, u64>::default();

    // Add nodes
    let node_index = nodes
        .iter()
        .map(|n| (n.clone(), graph.add_node(n.clone())))
        .collect::<HashMap<_, _>>();

    // Add edges
    input.iter().for_each(|i| {
        for to in i.to.iter() {
            graph.add_edge(node_index[&i.from], node_index[to], 1);
        }
    });

    let min: rustworkx_core::Result<Option<(usize, Vec<_>)>> =
        stoer_wagner_min_cut(&graph, |_| Ok(1));

    let (cut_size, node_list) = min.unwrap().unwrap();
    assert_eq!(cut_size, 3);

    let list_len = node_list.len() as u64;

    list_len * (nodes.len() as u64 - list_len)
}

fn get_nodes(input: &[Conn]) -> Vec<String> {
    let mut nodes = input
        .iter()
        .flat_map(|i| {
            let mut v = i.to.clone();
            v.push(i.from.clone());
            v
        })
        .collect::<Vec<_>>();

    nodes.sort();
    nodes.dedup();

    nodes
}

// Input parsing

struct Conn {
    from: String,
    to: Vec<String>,
}

fn input_transform(line: String) -> Conn {
    let mut split1 = line.split(": ");

    let from = split1.next().unwrap().to_string();
    let to = split1.next().unwrap();

    let to = to
        .split_ascii_whitespace()
        .map(String::from)
        .collect::<Vec<_>>();

    Conn { from, to }
}

#[cfg(test)]
mod tests {
    use aoc::input::parse_test_vec;

    use super::*;

    const EXAMPLE1: &str = "\
jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";

    #[test]
    fn test1() {
        let input = parse_test_vec(EXAMPLE1, input_transform).unwrap();
        assert_eq!(part1(&input), 54);
    }
}
