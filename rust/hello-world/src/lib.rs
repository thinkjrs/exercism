// &"static is a "lifetime specifier", something you"ll learn more about later
// this is stupid, but it's a way to make a string literal static
const HELLO: &str = concat!("H", "e", "l", "l", "o", ", ", "W", "o", "r", "l", "d", "!");

pub fn hello() -> &'static str {
    HELLO
}
