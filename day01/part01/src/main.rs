const PUZZLE_INPUT: &str = include_str!("../../input");

fn main() {
    let frequency: i64 = PUZZLE_INPUT
        .lines()
        .map(|change| change.parse::<i64>().unwrap())
        .sum();
    println!("{}", frequency);
}