use std::{
    cmp::{max, min},
    collections::HashMap,
    error::Error,
    ops::RangeInclusive,
};

use aoc::input::parse_input_vec;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(19, input_transform)?;
    let (rules, parts) = parse_input(&input);

    // Run parts
    println!("Part 1: {}", part1(&rules, &parts));
    println!("Part 2: {}", part2(&rules));

    Ok(())
}

fn part1(rules: &HashMap<String, Rule>, parts: &[Part]) -> u64 {
    // Iterate parts
    parts
        .iter()
        .map(|part| {
            // Get first rule
            let mut rule = rules.get("in").unwrap();

            // Condition loop
            loop {
                // Iterate conditions in the rule
                let action = rule.conditions.iter().find_map(|cond| {
                    // Get the value from the part
                    let part_value = cond.term.get(part);

                    // Test the value
                    if match cond.op {
                        Op::Lt => part_value < cond.value,
                        Op::Gt => part_value > cond.value,
                    } {
                        // Passed - execute condition
                        Some(&cond.then)
                    } else {
                        None
                    }
                });

                // If no condition triggered then execute the else condition
                let action = match action {
                    Some(action) => action,
                    None => &rule.otherwise,
                };

                // Process the action
                match action {
                    Action::Accept => {
                        // Accept the part
                        break part.sum();
                    }
                    Action::Reject => {
                        // Reject the part
                        break 0;
                    }
                    Action::Goto(rule_name) => {
                        // Go to another rule
                        rule = rules.get(rule_name).unwrap();
                    }
                }
            }
        })
        .sum()
}

fn part2(rules: &HashMap<String, Rule>) -> u64 {
    // Accepted part ranges
    let mut accepted = Vec::new();

    // Process the first rule set
    process_rules(rules, "in", Default::default(), &mut accepted);

    // Sum up part combinations
    accepted.iter().map(|a| a.combinations()).sum()
}

#[derive(Debug, Clone)]
struct Ranges {
    ranges: Vec<RangeInclusive<u16>>,
}

impl Default for Ranges {
    fn default() -> Self {
        Self {
            ranges: vec![1..=4000; 4],
        }
    }
}

impl Ranges {
    /// Return the number of part combinations for this set of ranges
    fn combinations(&self) -> u64 {
        self.ranges
            .iter()
            .map(|r| r.clone().count() as u64)
            .product()
    }

    /// Splits a range with a given operation
    fn split(&mut self, term: &Term, op: &Op, value: u16) -> Ranges {
        // Clone self to split off range
        let mut split_ranges = self.clone();

        // Get pointers to required ranges
        let self_range = &mut self.ranges[*term as usize];
        let split_range = &mut split_ranges.ranges[*term as usize];

        // Adjust the split off range
        let (start, end) = match op {
            Op::Gt => (max(value + 1, *split_range.start()), *split_range.end()),
            Op::Lt => (*split_range.start(), min(*split_range.end(), value - 1)),
        };

        *split_range = start..=end;

        // Adjust this range
        let (start, end) = match op {
            Op::Lt => (max(value, *self_range.start()), *self_range.end()),
            Op::Gt => (*self_range.start(), min(*self_range.end(), value)),
        };

        *self_range = start..=end;

        // Return split off range
        split_ranges
    }
}

fn process_rules(
    rules: &HashMap<String, Rule>,
    rule: &str,
    mut ranges: Ranges,
    accepted: &mut Vec<Ranges>,
) {
    // Get the rule from the rule map
    let rule = rules.get(rule).unwrap();

    // Process each condition recursively
    rule.conditions
        .iter()
        .for_each(|rule| process_rule(rules, rule, &mut ranges, accepted));

    // Process the else action
    process_action(rules, &rule.otherwise, ranges, accepted);
}

fn process_rule(
    rules: &HashMap<String, Rule>,
    cond: &Condition,
    ranges: &mut Ranges,
    accepted: &mut Vec<Ranges>,
) {
    // Split the ranges according to this operation
    let this_ranges = ranges.split(&cond.term, &cond.op, cond.value);

    // Process the action with the split off range
    process_action(rules, &cond.then, this_ranges, accepted);
}

fn process_action(
    rules: &HashMap<String, Rule>,
    action: &Action,
    ranges: Ranges,
    accepted: &mut Vec<Ranges>,
) {
    // Process the action
    match action {
        Action::Accept => accepted.push(ranges),
        Action::Reject => (),
        Action::Goto(rule_name) => process_rules(rules, rule_name, ranges, accepted),
    }
}

// Input parsing

#[derive(Debug)]
struct Rule {
    conditions: Vec<Condition>,
    otherwise: Action,
}

#[derive(Debug)]
struct Condition {
    term: Term,
    op: Op,
    value: u16,
    then: Action,
}

#[derive(Debug, Clone, Copy)]
enum Term {
    X = 0,
    M,
    A,
    S,
}

impl Term {
    fn new(string: &str) -> Self {
        match string {
            "x" => Term::X,
            "m" => Term::M,
            "a" => Term::A,
            "s" => Term::S,
            term => panic!("Invalid term {term}"),
        }
    }

    fn get(&self, part: &Part) -> u16 {
        part.values[*self as usize]
    }
}

#[derive(Debug)]
enum Op {
    Gt,
    Lt,
}

#[derive(Debug)]
enum Action {
    Accept,
    Reject,
    Goto(String),
}

impl Action {
    fn new(string: &str) -> Self {
        match string {
            "A" => Action::Accept,
            "R" => Action::Reject,
            target => Action::Goto(target.to_string()),
        }
    }
}

#[derive(Debug, Default)]
struct Part {
    values: [u16; 4],
}

impl Part {
    fn sum(&self) -> u64 {
        self.values.iter().map(|v| *v as u64).sum()
    }
}

fn input_transform(line: String) -> String {
    line
}

fn parse_input(lines: &[String]) -> (HashMap<String, Rule>, Vec<Part>) {
    let mut rules = HashMap::new();
    let mut parts = Vec::new();

    let mut in_parts = false;

    for line in lines {
        if in_parts {
            let mut part = Part::default();

            for attr in line
                .trim_start_matches('{')
                .trim_end_matches('}')
                .split(',')
            {
                let mut split = attr.split('=');

                let term = Term::new(split.next().unwrap());
                let value = split.next().unwrap().parse::<u16>().unwrap();

                part.values[term as usize] = value;
            }

            parts.push(part);
        } else if line.is_empty() {
            in_parts = true;
        } else {
            let mut split1 = line.split('{');

            let name = split1.next().unwrap();
            let condition_str = split1.next().unwrap().trim_end_matches('}');

            let (conditions, otherwise) = condition_str.split(',').fold(
                (Vec::new(), None),
                |(mut conditions, mut otherwise), rule_str| {
                    if rule_str.contains(':') {
                        let mut split2 = rule_str.split(':');

                        let cond_str = split2.next().unwrap();

                        let term = Term::new(&cond_str[0..1]);

                        let op = match &cond_str[1..2] {
                            "<" => Op::Lt,
                            ">" => Op::Gt,
                            op => panic!("Invalid operator {op}"),
                        };

                        let value = cond_str[2..].parse::<u16>().unwrap();

                        let then = split2.next().map(Action::new).unwrap();

                        conditions.push(Condition {
                            term,
                            op,
                            value,
                            then,
                        })
                    } else {
                        otherwise = Some(Action::new(rule_str));
                    }

                    (conditions, otherwise)
                },
            );

            rules.insert(
                name.to_string(),
                Rule {
                    conditions,
                    otherwise: otherwise.unwrap(),
                },
            );
        }
    }

    (rules, parts)
}

#[cfg(test)]
mod tests {
    use aoc::input::parse_test_vec;

    use super::*;

    const EXAMPLE1: &str = "\
px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

    #[test]
    fn test1() {
        let input = parse_test_vec(EXAMPLE1, input_transform).unwrap();
        let (rules, parts) = parse_input(&input);

        assert_eq!(part1(&rules, &parts), 19114);
        assert_eq!(part2(&rules), 167409079868000);
    }
}
