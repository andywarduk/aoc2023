use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let times = [42, 68, 69, 85];
    let dists = [284, 1005, 1122, 1341];

    // Run parts
    println!("Part 1: {}", part1(&times, &dists));
    println!("Part 2: {}", part2(42_686_985, 284_100_511_221_341));

    Ok(())
}

fn part1(times: &[u64], dists: &[u64]) -> u64 {
    times
        .iter()
        .zip(dists)
        .map(|(time, best_dist)| {
            (1..*time)
                .filter(|t| calc_dist(*time, *t) > *best_dist)
                .count() as u64
        })
        .product()
}

fn part2(time: u64, dist: u64) -> u64 {
    // Find first win
    let first = (1..time).find(|t| calc_dist(time, *t) > dist).unwrap();

    // Find last win
    let last = (1..time)
        .rev()
        .find(|t| calc_dist(time, *t) > dist)
        .unwrap();

    (last - first) + 1
}

fn calc_dist(total_time: u64, press_time: u64) -> u64 {
    let speed = press_time;
    let time_left = total_time - press_time;

    time_left * speed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let times = [7, 15, 30];
        let dists = [9, 40, 200];

        assert_eq!(part1(&times, &dists), 288);
        assert_eq!(part2(71530, 940200), 71503);
    }
}
