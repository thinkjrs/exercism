pub fn raindrops(n: u32) -> String {
    let words = [(3, "Pling"), (5, "Plang"), (7, "Plong")];
    let result: String = words
        .iter()
        .filter_map(
            |&(number, word)| {
                if n % number == 0 {
                    Some(word)
                } else {
                    None
                }
            },
        )
        .collect();

    if result.is_empty() {
        n.to_string()
    } else {
        result
    }
}
