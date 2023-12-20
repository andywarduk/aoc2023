use std::{error::Error, fs::File, io::Write};

use aoc::input::parse_input_vec;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(20, input_transform)?;
    let (components, connections) = parse_input(&input);

    let mut file = File::create("vis/day20.dot")?;

    file.write_fmt(format_args!("digraph {{\n"))?;

    let node = |file: &mut File, name, comp: &Component| -> Result<(), Box<dyn Error>> {
        file.write_fmt(format_args!("{name} [label=\"{name}\\n{comp:?}\""))?;

        let shape = match comp {
            Component::Conjunction => "box",
            Component::Button => "invhouse",
            Component::Broadcaster => "hexagon",
            Component::FlipFlop => "oval",
            Component::Output => "house",
        };

        file.write_fmt(format_args!(" shape=\"{shape}\""))?;

        file.write_fmt(format_args!("];\n"))?;

        Ok(())
    };

    // Button & broadcaster first
    file.write_fmt(format_args!("subgraph {{\n"))?;

    for (name, comp) in &components {
        if !matches!(comp, Component::Button | Component::Broadcaster) {
            continue;
        }

        node(&mut file, name, comp)?;
    }

    file.write_fmt(format_args!("}}\n"))?;

    // FlipFlops next
    file.write_fmt(format_args!("subgraph {{\n"))?;

    for (name, comp) in &components {
        if !matches!(comp, Component::FlipFlop) {
            continue;
        }

        node(&mut file, name, comp)?;
    }

    file.write_fmt(format_args!("}}\n"))?;

    // Conjunctions next
    file.write_fmt(format_args!("subgraph {{\n"))?;

    for (name, comp) in &components {
        if !matches!(comp, Component::Conjunction) {
            continue;
        }

        node(&mut file, name, comp)?;
    }

    file.write_fmt(format_args!("}}\n"))?;

    // Outputs
    file.write_fmt(format_args!("subgraph {{\n"))?;

    for (name, comp) in &components {
        if !matches!(comp, Component::Output) {
            continue;
        }

        node(&mut file, name, comp)?;
    }

    file.write_fmt(format_args!("}}\n"))?;

    // Connections
    for (f, t) in connections {
        file.write_fmt(format_args!("{f} -> {t}\n"))?;
    }

    file.write_fmt(format_args!("}}\n"))?;

    Ok(())
}

#[derive(Debug, Clone, PartialEq)]
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

type CompConn = (Vec<(String, Component)>, Vec<(String, String)>);

fn parse_input(input: &[String]) -> CompConn {
    let mut components = Vec::new();
    let mut connections = Vec::new();

    // Add button
    components.push(("button".to_string(), Component::Button));

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

        components.push((name.to_string(), comp));
    }

    connections.push(("button".to_string(), "broadcaster".to_string()));

    // Add outputs if required
    connections.iter().map(|(_, b)| b).for_each(|b| {
        if !components.iter().any(|(n, _)| n == b) {
            components.push((b.to_string(), Component::Output))
        }
    });

    (components, connections)
}
