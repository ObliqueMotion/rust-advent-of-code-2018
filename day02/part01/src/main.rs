const PUZZLE_INPUT: &str = include_str!("../../input");

fn letter_count(id: &str) -> [u8; 26] {
    id.chars()
        .map(|letter| usize::from(letter as u8 - b'a'))
        .fold([0; 26], |mut count, letter| {
            count[letter] += 1;
            count
        })
}

fn has_repeated_letter(n: u8) -> impl Fn(&[u8; 26]) -> bool {
    move |letter_count| letter_count.iter().any(|&count| count == n)
}

fn main() {
    let ids_iter = PUZZLE_INPUT.lines().map(letter_count);
    let num_ids_with_double = ids_iter.clone().filter(has_repeated_letter(2)).count();
    let num_ids_with_triple = ids_iter.filter(has_repeated_letter(3)).count();
    println!("{}", num_ids_with_double * num_ids_with_triple);
}