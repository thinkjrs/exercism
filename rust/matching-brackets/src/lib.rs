use std::collections::HashMap;

pub fn brackets_are_balanced(string: &str) -> bool {
    let default_matches = HashMap::from([('{', '}'), ('[', ']'), ('(', ')')]);
    let mut stack: Vec<&char> = vec![];

    for c in string
        .chars()
        .filter(|&c| default_matches.contains_key(&c) || default_matches.values().any(|&v| v == c))
    {
        if let Some(expected_closing) = default_matches.get(&c) {
            stack.push(expected_closing);
        } else if stack.pop() != Some(&c) {
            return false;
        }
    }

    stack.is_empty()
}
