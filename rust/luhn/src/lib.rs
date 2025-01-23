/// Check a Luhn checksum.
pub fn double_digit(digit: u32) -> u32 {
    let doubled = digit << 1;
    doubled - (doubled > 9) as u32 * 9
}
pub fn is_valid(code: &str) -> bool {
    let code = code.replace(" ", "");
    if code.len() < 2 {
        return false;
    }
    code.chars()
        .rev()
        .enumerate()
        .try_fold(0_u32, |sum, (i, c)| {
            c.to_digit(10).map(|digit| {
                sum + if i % 2 == 1 {
                    double_digit(digit)
                } else {
                    digit
                }
            })
        })
        .map_or(false, |sum| sum % 10 == 0)
}
