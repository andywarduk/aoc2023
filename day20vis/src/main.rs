use std::{collections::HashMap, error::Error, fs::File, io::Write};

use aoc::input::parse_input_vec;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(20, input_transform)?;
    let (components, connections) = parse_input(&input);

    let mut file = File::create("vis/day20.dot")?;

    file.write_fmt(format_args!("digraph {{\n"))?;

    for (n, c) in components {
        file.write_fmt(format_args!("{n} [label=\"{n}\\n{c:?}\"]\n"))?;
    }

    for (f, t) in connections {
        file.write_fmt(format_args!("{f} -> {t}\n"))?;
    }

    file.write_fmt(format_args!("}}\n"))?;

    Ok(())
}

#[derive(Debug, Clone)]
enum Component {
    Button,
    Broadcaster,
    FlipFlop,
    Conjunction,
    Output,
}

// Input parsing

type InputEnt = String;

fn input_transform(line: String) -> InputEnt {
    line
}

fn parse_input(input: &[String]) -> (HashMap<String, Component>, Vec<(String, String)>) {
    let mut components = HashMap::new();
    let mut connections = Vec::new();

    for line in input {
        let mut split1 = line.split(" -> ");
        let comp = split1.next().unwrap();
        let conns = split1.next().unwrap();
        let conns = conns.split(", ").map(String::from).collect::<Vec<_>>();

        let (name, comp) = if &comp[0..1] == "&" {
            (&comp[1..], Component::Conjunction)
        } else if &comp[0..1] == "%" {
            (&comp[1..], Component::FlipFlop)
        } else if comp == "broadcaster" {
            (comp, Component::Broadcaster)
        } else {
            panic!("Invalid component")
        };

        conns
            .into_iter()
            .for_each(|conn| connections.push((name.to_string(), conn)));

        components.insert(name.to_string(), comp);
    }

    // Add button
    components.insert("button".to_string(), Component::Button);

    connections.push(("button".to_string(), "broadcaster".to_string()));

    // Add outputs if required
    let missing = connections
        .iter()
        .filter(|(_, b)| !components.contains_key(b))
        .map(|(a, b)| (a.to_owned(), b.to_owned()))
        .collect::<Vec<_>>();

    for (_, b) in missing {
        components.insert(b.clone(), Component::Output);
    }

    (components, connections)
}
