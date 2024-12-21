use std::collections::HashMap;

pub fn brackets_are_balanced(string: &str) -> bool {
    let openings = ['{', '[', '('];
    let closings = ['}', ']', ')'];
    let default_matches: HashMap<char, char> =
        openings.into_iter().zip(closings.into_iter()).collect();
    let mut stack: Vec<char> = vec![];
    let matches = string
        .chars()
        .filter_map(|s| match s {
            '{' | '[' | '(' | '}' | ']' | ')' => Some(s),
            _ => None,
        })
        .collect::<String>();
    if matches.len() % 2 != 0 {
        return false;
    }
    for c in matches.chars() {
        // check if opening and push to stack
        if openings.contains(&c) {
            stack.push(c);
            continue;
        }
        if closings.contains(&c) {
            if let Some(opening) = stack.pop() {
                if let Some(expected_closing) = default_matches.get(&opening) {
                    if expected_closing != &c {
                        return false;
                    }
                }
            } else {
                return false;
            }
        }
    }

    if stack.len() > 0 {
        false
    } else {
        true
    }
}
