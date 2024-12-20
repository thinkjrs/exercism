use std::{collections::HashMap, default};

fn split_in_half_by_chars(s: &str) -> (String, String) {
    let chars: Vec<char> = s.chars().collect();
    let mid = chars.len() / 2;

    let first_half: String = chars[..mid].iter().collect();
    let second_half: String = chars[mid..].iter().collect();

    (first_half, second_half)
}
pub fn brackets_are_balanced(string: &str) -> bool {
    let default_matches = HashMap::from([('{', '}'), ('[', ']'), ('(', ')')]);
    let results = string
        .chars()
        .filter_map(|s| match s {
            '{' | '[' | '(' | '}' | ']' | ')' => Some(s),
            _ => None,
        })
        .collect::<String>();
    if results.len() % 2 != 0 {
        return false;
    }
    let (lhs, rhs) = split_in_half_by_chars(&results);
    // split the string and iterate pairwise
    for (left, right) in lhs.chars().zip(rhs.chars().rev()) {
        if let Some(matched) = default_matches.get(&left) {
            if matched != &right {
                println!("Oh no not matched! {} {}", matched, right);
                return false;
            }
        }
    }
    true
}
