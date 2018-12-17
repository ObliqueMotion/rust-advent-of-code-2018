const PUZZLE_INPUT: &str = include_str!("../../input");

fn correct_pair(ids: &(&str, &str)) -> bool {
    let &(id1, id2) = ids;
    1 == id1
        .chars()
        .zip(id2.chars())
        .filter(|(c1, c2)| c1 != c2)
        .count()
}

fn common_letters(ids: (&str, &str)) -> String {
    let (id1, id2) = ids;
    id1.chars()
        .zip(id2.chars())
        .filter(|(c1, c2)| c1 == c2)
        .map(|(c, _)| c)
        .collect()
}

fn main() {
    let mut ids: Vec<&str> = PUZZLE_INPUT.lines().collect();
    ids.sort();
    match ids
        .iter()
        .cloned()
        .zip(ids.iter().cloned().skip(1))
        .find(correct_pair)
    {
        Some(pair) => println!("{}", common_letters(pair)),
        None => unreachable!(),
    }
}