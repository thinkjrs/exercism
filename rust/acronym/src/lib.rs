// split by whitespace or hyphens
// i personally hate this, it's super messy and not idiomatic
pub fn split(phrase: &str) -> Vec<&str> {
    let mut words = vec![];
    let mut start = 0;
    let chars: Vec<char> = phrase.chars().collect();

    for i in 1..chars.len() {
        if chars[i].is_whitespace() || chars[i] == '-' {
            if start < i {
                words.push(&phrase[start..i]);
            }
            start = i + 1;
        } else if chars[i].is_uppercase() && chars[i - 1].is_lowercase() {
            words.push(&phrase[start..i]);
            start = i;
        }
    }

    if start < phrase.len() {
        words.push(&phrase[start..]);
    }

    words
}

// collect first letters
pub fn first_char(phrases: &Vec<&str>) -> Vec<char> {
    phrases
        .iter()
        .filter_map(|phrase| phrase.chars().find(|&c| c.is_alphabetic()))
        .collect()
}

// join first letters
pub fn join(chars: &Vec<char>) -> String {
    chars.iter().collect()
}

pub fn abbreviate(phrase: &str) -> String {
    //todo!("Given the phrase '{phrase}', return its acronym");

    let splits = split(phrase);
    let first_chars = first_char(&splits);
    join(&first_chars).to_uppercase()
}
