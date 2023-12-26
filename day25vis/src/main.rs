use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fs::File,
    io::Write,
};

use aoc::input::{parse_input_vec, parse_test_vec};

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

fn main() -> Result<(), Box<dyn Error>> {
    let input = parse_test_vec(EXAMPLE1, input_transform).unwrap();
    output(&input, "vis/day25ex.dot")?;

    let input = parse_input_vec(25, input_transform)?;
    output(&input, "vis/day25.dot")?;

    Ok(())
}

fn output(input: &[Conn], file: &str) -> Result<(), Box<dyn Error>> {
    let mut set: HashMap<String, Vec<String>> = HashMap::new();

    for c in input.iter() {
        for to in c.to.iter() {
            let from = c.from.clone();
            let to = to.clone();

            set.entry(from.clone())
                .and_modify(|v| v.push(to.clone()))
                .or_insert(vec![to.clone()]);

            set.entry(to)
                .and_modify(|v| v.push(from.clone()))
                .or_insert(vec![from.clone()]);
        }
    }

    for (k, v) in &set {
        if v.len() == 1 {
            println!("{k} -- {}", v[0]);
        }
    }

    let mut file = File::create(file).unwrap();

    file.write_fmt(format_args!("graph {{\n"))?;

    let mut output = HashSet::new();

    for c in input.iter() {
        let v = set.get(&c.from).unwrap();

        for to in v {
            let mut conn = vec![c.from.clone(), to.clone()];

            conn.sort();

            if !output.contains(&conn) {
                file.write_fmt(format_args!("{} -- {}\n", c.from, to))?;
                output.insert(conn);
            }
        }
    }

    file.write_fmt(format_args!("}}\n"))?;

    Ok(())
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
