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

fn part1(rules_map: &HashMap<String, Rules>, parts: &[Part]) -> u64 {
    parts
        .iter()
        .map(|part| {
            let mut rules = rules_map.get("in").unwrap();

            'outer: loop {
                'inner: for rule in rules.rules.iter() {
                    let action = match rule {
                        Rule::Action(action) => Some(action),
                        Rule::Condition(cond) => {
                            let part_value = cond.term.get(part);

                            if match cond.op {
                                Op::Lt => part_value < cond.value,
                                Op::Gt => part_value > cond.value,
                            } {
                                Some(&cond.then)
                            } else {
                                None
                            }
                        }
                    };

                    if let Some(action) = action {
                        match action {
                            Action::Accept => break 'outer part.sum(),
                            Action::Reject => break 'outer 0,
                            Action::Goto(rule) => {
                                rules = rules_map.get(rule).unwrap();
                                break 'inner;
                            }
                        }
                    }
                }
            }
        })
        .sum()
}

fn part2(rules_map: &HashMap<String, Rules>) -> u64 {
    let mut accepted = Vec::new();

    let mut ranges = Default::default();

    process_rules(rules_map, "in", &mut ranges, &mut accepted, Vec::new());

    //    println!("{accepted:?}");

    accepted
        .iter()
        .map(|a| {
            a.x_range.clone().count()
                * a.m_range.clone().count()
                * a.a_range.clone().count()
                * a.s_range.clone().count()
        })
        .sum::<usize>() as u64
}

#[derive(Debug, Clone)]
struct Ranges {
    x_range: RangeInclusive<u16>,
    m_range: RangeInclusive<u16>,
    a_range: RangeInclusive<u16>,
    s_range: RangeInclusive<u16>,
}

impl Default for Ranges {
    fn default() -> Self {
        Self {
            x_range: 1..=4000,
            m_range: 1..=4000,
            a_range: 1..=4000,
            s_range: 1..=4000,
        }
    }
}

impl Ranges {
    fn sum(&self) -> u64 {
        Self::range_sum(&self.x_range)
            + Self::range_sum(&self.m_range)
            + Self::range_sum(&self.a_range)
            + Self::range_sum(&self.s_range)
    }

    fn split(&mut self, term: &Term, op: &Op, value: u16) -> Ranges {
        let mut split_ranges = self.clone();

        let self_range = match term {
            Term::X => &mut self.x_range,
            Term::M => &mut self.m_range,
            Term::A => &mut self.a_range,
            Term::S => &mut self.s_range,
        };

        let split_range = match term {
            Term::X => &mut split_ranges.x_range,
            Term::M => &mut split_ranges.m_range,
            Term::A => &mut split_ranges.a_range,
            Term::S => &mut split_ranges.s_range,
        };

        let (start, end) = match op {
            Op::Gt => (max(value + 1, *split_range.start()), *split_range.end()),
            Op::Lt => (*split_range.start(), min(*split_range.end(), value - 1)),
        };

        *split_range = start..=end;

        let (start, end) = match op {
            Op::Lt => (max(value, *self_range.start()), *self_range.end()),
            Op::Gt => (*self_range.start(), min(*self_range.end(), value)),
        };

        *self_range = start..=end;

        split_ranges
    }

    fn all(&mut self) -> Ranges {
        let cloned = self.clone();

        let start = 1;
        let end = 0;
        self.x_range = start..=end;
        self.m_range = start..=end;
        self.a_range = start..=end;
        self.s_range = start..=end;

        cloned
    }

    fn adjust(&mut self, term: &Term, op: &Op, value: u16) -> bool {
        let range = match term {
            Term::X => &mut self.x_range,
            Term::M => &mut self.m_range,
            Term::A => &mut self.a_range,
            Term::S => &mut self.s_range,
        };

        let (start, end) = match op {
            Op::Gt => (max(value + 1, *range.start()), *range.end()),
            Op::Lt => (*range.start(), min(*range.end(), value - 1)),
        };

        if start <= end {
            *range = start..=end;
            true
        } else {
            false
        }
    }

    fn range_sum(range: &RangeInclusive<u16>) -> u64 {
        let a = *range.start() as u64;
        let b = *range.end() as u64;

        ((a + b) * (b - a + 1)) / 2
    }
}

fn process_rules(
    rules_map: &HashMap<String, Rules>,
    rule: &str,
    ranges: &mut Ranges,
    accepted: &mut Vec<Ranges>,
    mut path: Vec<String>,
) {
    path.push(format!("rule {}", rule));

    let rules = rules_map.get(rule).unwrap();

    rules
        .rules
        .iter()
        .for_each(|rule| process_rule(rules_map, rule, ranges, accepted, path.clone()));
}

fn process_rule(
    rules_map: &HashMap<String, Rules>,
    rule: &Rule,
    ranges: &mut Ranges,
    accepted: &mut Vec<Ranges>,
    mut path: Vec<String>,
) {
    let (mut this_ranges, action) = match rule {
        Rule::Condition(cond) => {
            let action = &cond.then;

            path.push(format!("{:?} {:?} {}", cond.term, cond.op, cond.value));

            let this_ranges = ranges.split(&cond.term, &cond.op, cond.value);

            (this_ranges, action)
        }
        Rule::Action(action) => (ranges.all(), action),
    };

    match action {
        Action::Accept => {
            println!("Accept {:?} via {:?}", this_ranges, path);

            accepted.push(this_ranges)
        }
        Action::Reject => (),
        Action::Goto(rules) => process_rules(rules_map, rules, &mut this_ranges, accepted, path),
    }
}

// Input parsing

#[derive(Debug)]
struct Rules {
    rules: Vec<Rule>,
}

#[derive(Debug)]
enum Rule {
    Condition(Condition),
    Action(Action),
}

#[derive(Debug)]
struct Condition {
    term: Term,
    op: Op,
    value: u16,
    then: Action,
}

#[derive(Debug)]
enum Term {
    X,
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
        match self {
            Term::X => part.x,
            Term::M => part.m,
            Term::A => part.a,
            Term::S => part.s,
        }
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
    x: u16,
    m: u16,
    a: u16,
    s: u16,
}

impl Part {
    fn sum(&self) -> u64 {
        self.x as u64 + self.m as u64 + self.a as u64 + self.s as u64
    }
}

type InputEnt = String;

fn input_transform(line: String) -> InputEnt {
    line
}

fn parse_input(lines: &[String]) -> (HashMap<String, Rules>, Vec<Part>) {
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

                let term = split.next().unwrap();
                let value = split.next().unwrap().parse::<u16>().unwrap();

                match term {
                    "x" => part.x = value,
                    "m" => part.m = value,
                    "a" => part.a = value,
                    "s" => part.s = value,
                    term => panic!("Invalid term {term}"),
                }
            }

            parts.push(part);
        } else if line.is_empty() {
            in_parts = true;
        } else {
            let mut split1 = line.split('{');

            let name = split1.next().unwrap();
            let condition_str = split1.next().unwrap().trim_end_matches('}');

            let rules_vec = condition_str
                .split(',')
                .map(|rule_str| {
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

                        Rule::Condition(Condition {
                            term,
                            op,
                            value,
                            then,
                        })
                    } else {
                        Rule::Action(Action::new(rule_str))
                    }
                })
                .collect::<Vec<_>>();

            rules.insert(name.to_string(), Rules { rules: rules_vec });
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
        // 115,264,000,000,000
        // 167,409,079,868,000
        // 400,000,000,000,000
        assert_eq!(part2(&rules), 167409079868000);
    }

    #[test]
    fn range_sum_test() {
        let range = 1..=5;
        assert_eq!(Ranges::range_sum(&range), 15);

        let range = 2..=5;
        assert_eq!(Ranges::range_sum(&range), 14);
    }
}
