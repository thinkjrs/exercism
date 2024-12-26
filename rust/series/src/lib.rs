pub fn series(digits: &str, len: usize) -> Vec<String> {
    let mut digit_series = Vec::new();

    match len > digits.len() {
        true => digit_series,
        _ => {
            for i in 0..=digits.len() {
                match i + len {
                    len if len > digits.len() => break,
                    _ => digit_series.push(digits[i..i + len].to_string()),
                }
            }
            digit_series
        }
    }
}
