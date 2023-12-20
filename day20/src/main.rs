use std::{
    collections::{HashMap, VecDeque},
    error::Error,
};

use aoc::input::parse_input_vec;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(20, input_transform)?;
    let (components, connections) = parse_input(&input);

    // Run parts
    println!("Part 1: {}", part1(components.clone()));
    println!("Part 2: {}", part2(components, connections));

    Ok(())
}

fn part1(mut components: HashMap<String, Component>) -> u64 {
    let mut low = 0;
    let mut high = 0;

    let mut pulse_queue = VecDeque::new();

    for _ in 0..1000 {
        // Push the button
        push_button(&mut components, &mut pulse_queue);

        // Loop queue entries
        while let Some(pulse) = pulse_queue.pop_front() {
            // Count pulses
            match pulse.pulse {
                Pulse::Low => low += 1,
                Pulse::High => high += 1,
            }

            // Process the pulse
            process_pulse(&pulse, &mut components, &mut pulse_queue)
        }
    }

    low * high
}

fn part2(mut components: HashMap<String, Component>, connections: Vec<(String, String)>) -> u64 {
    let mut presses: u64 = 0;
    let mut pulse_queue = VecDeque::new();

    // Component just before rx should be conjunction
    let last = connections
        .iter()
        .find_map(|(a, b)| if b == "rx" { Some(a.to_string()) } else { None })
        .expect("Unable to find last component");

    // Number of button presses (cycles) for each conjunction input to go high
    let mut cycles = match components.get(&last) {
        Some(Component::Conjunction(conj)) => vec![None; conj.state.len()],
        _ => panic!("Unable to get last conjunction"),
    };

    'outer: loop {
        // Push the button
        push_button(&mut components, &mut pulse_queue);
        presses += 1;

        // Loop queue entries
        while let Some(pulse) = pulse_queue.pop_front() {
            // Process the pulse
            process_pulse(&pulse, &mut components, &mut pulse_queue);

            // Last component target?
            if pulse.target == last {
                match components.get(&last) {
                    Some(Component::Conjunction(conj)) => {
                        // Compare conjunction state with the cycle counts
                        cycles
                            .iter_mut()
                            .zip(&conj.state)
                            .filter(|(c, (_, p))| **p == Pulse::High && c.is_none())
                            .for_each(|(c, _)| *c = Some(presses));

                        // Got cycle count for all inputs?
                        if !cycles.iter().any(|c| c.is_none()) {
                            // Yes - finished
                            break 'outer;
                        }
                    }
                    _ => panic!("Unable to get last component"),
                }
            }
        }
    }

    // Find lowest common multiplier for cycle counts and return
    cycles
        .iter()
        .skip(1)
        .fold(cycles[0].unwrap(), |acc, e| lcm(acc, e.unwrap()))
}

#[derive(Debug)]
struct WirePulse {
    source: String,
    target: String,
    pulse: Pulse,
}

/// Push the button!
fn push_button(components: &mut HashMap<String, Component>, pulse_queue: &mut VecDeque<WirePulse>) {
    // Get button component
    match components.get("button") {
        Some(Component::Button(button)) => {
            // Send a low pulse to the connected component
            send_pulse(pulse_queue, "button", &button.cout, Pulse::Low);
        }
        _ => panic!("Can't find button"),
    }
}

/// Process a pulse
fn process_pulse(
    pulse: &WirePulse,
    components: &mut HashMap<String, Component>,
    pulse_queue: &mut VecDeque<WirePulse>,
) {
    // Get the target component
    let comp = components
        .get_mut(&pulse.target)
        .unwrap_or_else(|| panic!("Can't find component {}", pulse.target));

    match comp {
        Component::Broadcaster(broadcaster) => {
            // Send a pulse to all connected components
            broadcaster.cout.iter().for_each(|target| {
                send_pulse(pulse_queue, &pulse.target, target, pulse.pulse);
            })
        }
        Component::Button(_) => panic!("Button in loop?"),
        Component::FlipFlop(flipflop) => {
            // Only act on low pulse
            if pulse.pulse == Pulse::Low {
                // Flip the state
                flipflop.state = !flipflop.state;

                // Work out type of pulse to send
                let out_pulse = if flipflop.state {
                    Pulse::High
                } else {
                    Pulse::Low
                };

                // Send the pulse to all connected components
                flipflop.cout.iter().for_each(|target| {
                    send_pulse(pulse_queue, &pulse.target, target, out_pulse);
                })
            }
        }
        Component::Conjunction(conj) => {
            // Get pointer to the input
            *conj
                .state
                .get_mut(&pulse.source)
                .expect("Conjunction pulse from unknown source") = pulse.pulse;

            // Work out output pulse type (Low if all inputs high, otherwise High)
            let out_pulse = if conj.state.iter().all(|(_, p)| *p == Pulse::High) {
                Pulse::Low
            } else {
                Pulse::High
            };

            // Send the pulse to all connected components
            conj.cout.iter().for_each(|target| {
                send_pulse(pulse_queue, &pulse.target, target, out_pulse);
            })
        }
        Component::Output(output) => output.pulses.push(pulse.pulse),
    }
}

/// Send a pulse from source to target
fn send_pulse(pulse_queue: &mut VecDeque<WirePulse>, source: &str, target: &str, pulse: Pulse) {
    pulse_queue.push_back(WirePulse {
        source: source.to_string(),
        target: target.to_string(),
        pulse,
    })
}

/// Lowest common multiple
// From https://en.wikipedia.org/wiki/Least_common_multiple
fn lcm(l: u64, r: u64) -> u64 {
    (l * r) / gcd(l, r)
}

/// Greatest common denominator
// From https://en.wikipedia.org/wiki/Binary_GCD_algorithm
pub fn gcd(mut u: u64, mut v: u64) -> u64 {
    let ored = u | v;

    if u == 0 || v == 0 {
        return ored;
    }

    // 'trailing_zeros' quickly counts a binary number's trailing zeros, giving its prime factorization's exponent on two
    let gcd_exponent_on_two = ored.trailing_zeros();

    // `>>=` divides the left by two to the power of the right, storing that in the left variable
    // `u` divided by its prime factorization's power of two turns it odd
    u >>= u.trailing_zeros();
    v >>= v.trailing_zeros();

    while u != v {
        if u < v {
            // Swap the variables' values with each other.
            core::mem::swap(&mut u, &mut v);
        }
        u -= v;
        u >>= u.trailing_zeros();
    }

    // `<<` multiplies the left by two to the power of the right
    u << gcd_exponent_on_two
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Pulse {
    Low,
    High,
}

#[derive(Debug, Clone)]
enum Component {
    Button(Button),
    Broadcaster(Broadcaster),
    FlipFlop(FlipFlop),
    Conjunction(Conjunction),
    Output(Output),
}

/// Sends a pulse to the broadcaster
#[derive(Debug, Clone)]
struct Button {
    cout: String,
}

/// Sends in pulse to all outputs
#[derive(Debug, Clone)]
struct Broadcaster {
    cout: Vec<String>,
}

/// (%) Switches state on low pulse and sends low if off and high if on
#[derive(Debug, Clone)]
struct FlipFlop {
    state: bool,
    cout: Vec<String>,
}

/// (&) Sends low if all inputs are high, else low
#[derive(Debug, Clone)]
struct Conjunction {
    state: HashMap<String, Pulse>,
    cout: Vec<String>,
}

/// Remembers received pulses
#[derive(Debug, Clone)]
struct Output {
    pulses: Vec<Pulse>,
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
            (
                &comp[1..],
                Component::Conjunction(Conjunction {
                    state: HashMap::new(),
                    cout: conns.clone(),
                }),
            )
        } else if &comp[0..1] == "%" {
            (
                &comp[1..],
                Component::FlipFlop(FlipFlop {
                    state: false,
                    cout: conns.clone(),
                }),
            )
        } else if comp == "broadcaster" {
            (
                comp,
                Component::Broadcaster(Broadcaster {
                    cout: conns.clone(),
                }),
            )
        } else {
            panic!("Invalid component")
        };

        conns
            .into_iter()
            .for_each(|conn| connections.push((name.to_string(), conn)));

        components.insert(name.to_string(), comp);
    }

    // Add button
    components.insert(
        "button".to_string(),
        Component::Button(Button {
            cout: "broadcaster".to_string(),
        }),
    );

    connections.push(("button".to_string(), "broadcaster".to_string()));

    // Add outputs if required
    connections.iter().map(|(_, b)| b).for_each(|b| {
        components
            .entry(b.to_string())
            .or_insert_with(|| Component::Output(Output { pulses: Vec::new() }));
    });

    // Resolve conjunctions
    components.iter_mut().for_each(|(name, comp)| {
        if let Component::Conjunction(conj) = comp {
            connections
                .iter()
                .filter(|(_, b)| b == name)
                .for_each(|(a, _)| {
                    conj.state.insert(a.to_string(), Pulse::Low);
                })
        }
    });

    (components, connections)
}

#[cfg(test)]
mod tests {
    use aoc::input::parse_test_vec;

    use super::*;

    const EXAMPLE1: &str = "\
broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";

    const EXAMPLE2: &str = "\
broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";

    #[test]
    fn test1() {
        let input = parse_test_vec(EXAMPLE1, input_transform).unwrap();
        let (components, _) = parse_input(&input);
        assert_eq!(part1(components), 32000000);
    }

    #[test]
    fn test2() {
        let input = parse_test_vec(EXAMPLE2, input_transform).unwrap();
        let (components, _) = parse_input(&input);
        assert_eq!(part1(components), 11687500);
    }
}
