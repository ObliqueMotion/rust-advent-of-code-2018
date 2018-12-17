use std::collections::HashMap;
const PUZZLE_INPUT: &str = include_str!("../../input");

type Point = (usize,usize);

// Example: "#206 @ 163,522: 17x11" -> (163, 522, 17, 11)
fn parse_claim(claim: &str) -> (usize,usize,usize,usize) {
    let claim: Vec<usize> = claim
        .split(|c: char| !c.is_digit(10))
        .filter(|s| !s.is_empty())
        .skip(1)
        .map(|num| num.parse::<usize>().unwrap())
        .collect();
    (claim[0],claim[1],claim[2],claim[3])
}

// Example: (5, 7, 2, 2) -> [(5,7), (5,8), (6,7), (6,8)]
fn expand_claim(claim: (usize,usize,usize,usize)) -> Vec<Point> {
    let (x_start,y_start,x_dim,y_dim) = claim;
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