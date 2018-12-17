const PUZZLE_INPUT: &str = include_str!("../../input");

type Claim = (usize,usize,usize,usize);

fn parse_claim(claim: &str) -> Claim {
    let claim: Vec<usize> = claim.split(|c: char| !c.is_digit(10))
        .filter(|s| !s.is_empty())
        .skip(1)
        .map(|num| num.parse::<usize>().unwrap())
        .collect();
    (claim[0],claim[1],claim[2],claim[3])
}

fn do_not_overlap<'a>(claim1: &'a Claim) -> impl Fn(&'a Claim) -> bool {
    move |claim2| {
        let &(x1_start, y1_start, x1_dim, y1_dim) = claim1;
        let &(x2_start, y2_start, x2_dim, y2_dim) = claim2;
        claim1 == claim2
        || x1_start > x2_start + x2_dim
        || y1_start > y2_start + y2_dim
        || x2_start > x1_start + x1_dim
        || y2_start > y1_start + y1_dim
    }
}

fn main() {
    let claims: Vec<Claim> = PUZZLE_INPUT
        .lines()
        .map(parse_claim)
        .collect();
    
    for claim in &claims {
        if claims.iter().all(do_not_overlap(claim)) {
            println!("{:?}", claim);
        }
    }
}