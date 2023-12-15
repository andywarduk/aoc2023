use std::error::Error;

use aoc::input::parse_input_line;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_line(15, input_transform)?;

    // Run parts
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

fn part1(input: &str) -> u64 {
    input.split(',').map(hash).sum()
}

#[derive(Debug, Clone)]
struct Lens {
    code: String,
    fl: u8,
}

fn part2(input: &str) -> u64 {
    let mut boxes = vec![Vec::new(); 256];

    input.split(',').for_each(|i| {
        if i.contains('-') {
            // Get code
            let code = i.split('-').next().unwrap();

            // Calculate box number
            let box_no = hash(code) as usize;

            // Does box contain thsi lens?
            if let Some(index) = boxes[box_no]
                .iter()
                .position(|lens: &Lens| lens.code == code)
            {
                // Yes - remove it
                boxes[box_no].remove(index);
            }
        } else {
            // Extract terms
            let mut split = i.split('=');

            let code = split.next().unwrap().to_string();
            let fl = split.next().unwrap().parse::<u8>().unwrap();

            // Calculate box number
            let box_no = hash(&code) as usize;

            // Does box already contain this code?
            if let Some(lens) = boxes[box_no].iter_mut().find(|ent| ent.code == code) {
                // Yes - change it
                lens.fl = fl
            } else {
                // No - add it
                boxes[box_no].push(Lens { code, fl });
            }
        }
    });

    // Calculate focal power
    boxes
        .iter()
        .enumerate()
        .map(|(box_no, box_vec)| {
            box_vec
                .iter()
                .enumerate()
                .map(|(pos, lens)| (box_no as u64 + 1) * (pos as u64 + 1) * lens.fl as u64)
                .sum::<u64>()
        })
        .sum()
}

fn hash(string: &str) -> u64 {
    string
        .chars()
        .fold(0, |acc, c| ((acc + c as u64) * 17) % 256)
}

// Input parsing

type InputEnt = String;

fn input_transform(line: String) -> InputEnt {
    line
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test1() {
        assert_eq!(part1(EXAMPLE1), 1320);
        assert_eq!(part2(EXAMPLE1), 145);
    }
}
