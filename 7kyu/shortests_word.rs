//DESCRIPTION:
// Simple, given a string of words, return the length of the shortest word(s).

// String will never be empty and you do not need to account for different data types.

//MY SOLUTION:
fn find_short(s: &str) -> u32 {
    *s.split_whitespace()
        .map(|word| word.len() as u32)
        .collect::<Vec<u32>>()
        .iter()
        .min()
        .unwrap()
}
