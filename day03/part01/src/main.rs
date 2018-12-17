use std::collections::HashMap;
const PUZZLE_INPUT: &str = include_str!("../../input");

type Point = (usize,usize);

fn parse_claim(claim: &str) -> Vec<usize> {
    claim.split(|c: char| !c.is_digit(10))
        .filter(|s| !s.is_empty())
        .skip(1)
        .map(|num| num.parse::<usize>().unwrap())
        .collect()
}

fn expand_claim(claim: Vec<usize>) -> Vec<Point> {
    let x_start = claim[0];
    let y_start = claim[1];
    let x_dim   = claim[2];
    let y_dim   = claim[3];
    let mut squares = Vec::with_capacity(x_dim * y_dim);
    for x in x_start..x_start + x_dim {
        for y in y_start..y_start+y_dim {
            squares.push((x,y));
        }
    }
    squares
}

fn main() {
    let overlap_count = PUZZLE_INPUT
        .lines()
        .map(parse_claim)
        .map(expand_claim)
        .fold(HashMap::new(), |mut claims, claim| {
            for square in claim {
                claims.entry(square)
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            }
            claims
        })
        .iter()
        .filter(|&(_,&count)| count > 1)
        .count();
    println!("{}", overlap_count);
}