const PUZZLE_INPUT: &str = include_str!("../../input");

fn main() {
    let answer: i64 = PUZZLE_INPUT
        .lines()
        .map(|x| x.parse::<i64>().unwrap())
        .sum();
    println!("{}", answer);
}