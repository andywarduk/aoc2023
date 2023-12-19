use std::{collections::HashMap, error::Error, ops::RangeInclusive};

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
            let mut rule = rules.get("in").expect("'in' rule not found");

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
                        rule = rules
                            .get(rule_name)
                            .unwrap_or_else(|| panic!("Rule '{rule_name}' not found"));
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
    process_rule(rules, "in", Default::default(), &mut accepted);

    // Sum up part combinations
    accepted.iter().map(|a| a.combinations()).sum()
}

/// Ranges for each attribute
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
        // Get pointer to required range
        let self_range = &mut self.ranges[*term as usize];

        // Get range start and end
        let (start, end) = (*self_range.start(), *self_range.end());

        // Split the range according to operator
        let (split1, split2) = match op {
            Op::Gt => ((start..=value), (value + 1..=end)),
            Op::Lt => ((value..=end), (start..=(value - 1))),
        };

        // Update ranges
        *self_range = split1;

        // Clone self and set split off range
        let mut split_ranges = self.clone();
        split_ranges.ranges[*term as usize] = split2;

        // Return split
        split_ranges
    }
}

/// Process a rule
fn process_rule(
    rules: &HashMap<String, Rule>,
    rule_name: &str,
    mut ranges: Ranges,
    accepted: &mut Vec<Ranges>,
) {
    // Get the rule from the rule map
    let rule = rules
        .get(rule_name)
        .unwrap_or_else(|| panic!("Rule '{rule_name}' not found"));

    // Process each condition recursively
    rule.conditions
        .iter()
        .for_each(|rule| process_condition(rules, rule, &mut ranges, accepted));

    // Process the else action
    process_action(rules, &rule.otherwise, ranges, accepted);
}

/// Process a comdition
fn process_condition(
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

/// Process an action
fn process_action(
    rules: &HashMap<String, Rule>,
    action: &Action,
    ranges: Ranges,
    accepted: &mut Vec<Ranges>,
) {
    match action {
        Action::Accept => accepted.push(ranges),
        Action::Reject => (),
        Action::Goto(rule_name) => process_rule(rules, rule_name, ranges, accepted),
    }
}

// Input parsing

/// Rule with conditions and else clause
#[derive(Debug)]
struct Rule {
    conditions: Vec<Condition>,
    otherwise: Action,
}

/// Condition with test and action if true
#[derive(Debug)]
struct Condition {
    term: Term,
    op: Op,
    value: u16,
    then: Action,
}

/// Product terms
#[derive(Debug, Clone, Copy)]
enum Term {
    X = 0,
    M,
    A,
    S,
}

impl Term {
    /// Create product term from string
    fn new(string: &str) -> Self {
        match string {
            "x" => Term::X,
            "m" => Term::M,
            "a" => Term::A,
            "s" => Term::S,
            term => panic!("Invalid term {term}"),
        }
    }

    /// Gets the term value from a product
    fn get(&self, part: &Part) -> u16 {
        part.values[*self as usize]
    }
}

/// Test operators
#[derive(Debug)]
enum Op {
    Gt,
    Lt,
}

impl Op {
    /// Create operator from string
    fn new(op: &str) -> Self {
        match op {
            "<" => Op::Lt,
            ">" => Op::Gt,
            op => panic!("Invalid operator {op}"),
        }
    }
}

/// Actions
#[derive(Debug)]
enum Action {
    Accept,
    Reject,
    Goto(String),
}

impl Action {
    /// Create new action from a string
    fn new(string: &str) -> Self {
        match string {
            "A" => Action::Accept,
            "R" => Action::Reject,
            target => Action::Goto(target.to_string()),
        }
    }
}

/// Part with terms indexed by Term
#[derive(Debug, Default)]
struct Part {
    values: [u16; 4],
}

impl Part {
    /// Sums a product terms
    fn sum(&self) -> u64 {
        self.values.iter().map(|v| *v as u64).sum()
    }
}

fn input_transform(line: String) -> String {
    line
}

/// Parses input lines to rule hash map and product vector
fn parse_input(lines: &[String]) -> (HashMap<String, Rule>, Vec<Part>) {
    let mut rules = HashMap::new();
    let mut parts = Vec::new();

    let mut in_parts = false;

    for line in lines {
        if in_parts {
            // In parts section
            let mut part = Part::default();

            for attr in line
                .trim_start_matches('{')
                .trim_end_matches('}')
                .split(',')
            {
                let mut split = attr.split('=');

                let term = Term::new(split.next().expect("term not found"));
                let value = split
                    .next()
                    .expect("Part value not found")
                    .parse::<u16>()
                    .expect("Part value does not parse");

                part.values[term as usize] = value;
            }

            parts.push(part);
        } else if line.is_empty() {
            // Move to parts section
            in_parts = true;
        } else {
            // In Rules
            let mut split1 = line.split('{');

            let name = split1.next().expect("Name not found");
            let condition_str = split1
                .next()
                .expect("Condition clause not found")
                .trim_end_matches('}');

            let (conditions, otherwise) = condition_str.split(',').fold(
                (Vec::new(), None),
                |(mut conditions, mut otherwise), rule_str| {
                    if rule_str.contains(':') {
                        // Condition
                        let mut split2 = rule_str.split(':');
                        let cond_str = split2.next().expect("Condition not found");

                        let term = Term::new(&cond_str[0..1]);
                        let op = Op::new(&cond_str[1..2]);
                        let value = cond_str[2..]
                            .parse::<u16>()
                            .expect("Condition value does not parse");

                        let then = split2.next().map(Action::new).expect("Action not found");

                        conditions.push(Condition {
                            term,
                            op,
                            value,
                            then,
                        })
                    } else {
                        // Else clause
                        otherwise = Some(Action::new(rule_str));
                    }

                    (conditions, otherwise)
                },
            );

            rules.insert(
                name.to_string(),
                Rule {
                    conditions,
                    otherwise: otherwise.expect("No condition else found"),
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
