fn is_uppercase(message: &str) -> bool {
    message.chars().all(|c: char| !c.is_lowercase())
}

fn is_only_text(message: &str) -> bool {
    message.strip_suffix("?").unwrap().chars().all(|c: char| {
        c.is_alphabetic()
            || c.is_whitespace()
            || c.eq(&',')
            || c.eq(&'\'')
            || c.eq(&'!')
            || c.eq(&'.')
            || c.eq(&'\"')
    })
}

fn is_numeric_list(message: &str) -> bool {
    message
        .split(',')
        .map(str::trim)
        .all(|c: &str| c.chars().all(|c: char| c.is_numeric()))
}

pub fn reply(message: &str) -> &str {
    let message = message.trim();
    match message {
        _ if message.is_empty() => return "Fine. Be that way!",
        _ if is_uppercase(message) && message.ends_with('?') && is_only_text(message) => {
            return "Calm down, I know what I'm doing!"
        }
        _ if message.ends_with("?") => return "Sure.",
        _ if is_uppercase(message) && !message.ends_with('?') && !is_numeric_list(message) => {
            return "Whoa, chill out!"
        }
        _ => return "Whatever.",
    }
}
