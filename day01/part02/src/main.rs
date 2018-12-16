use std::collections::HashSet;
const PUZZLE_INPUT: &str = include_str!("../../input");

fn main() {
    let mut seen: HashSet<i64> = HashSet::new();
    seen.insert(0);
    let frequencies = PUZZLE_INPUT
        .lines()
        .map(|change| change.parse::<i64>().unwrap())
        .cycle()
        .scan(0, |frequency, change| {
            *frequency += change;
            Some(*frequency)
        });

    for frequency in frequencies {
        if seen.contains(&frequency) {
            println!("{}", frequency);
            return;
        }
        seen.insert(frequency);
    }
}