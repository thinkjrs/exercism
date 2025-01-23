/// Check a Luhn checksum.
pub fn double_digit(mut digit: u32) -> u32 {
    digit *= 2;
    if digit > 9 {
        digit -= 9;
    }
    digit
}
pub fn is_valid(code: &str) -> bool {
    //todo!("Is the Luhn checksum for {code} valid?");
    // Remove all spaces
    // Check if the length is less than 2, if truthy return false
    // Check if all characters are digits, if not return false
    // Initialize sum = 0
    // Loop from back to front, if the index is even, double the digit
    //   if doubling the digit results in a number greater than 9, subtract 9
    //   add to sum
    // Return if sum % 10 == 0
    let binding = code.replace(" ", "");
    let code = binding.chars();
    if code.as_str().len() < 2 {
        false
    } else {
        let mut sum = 0;
        for (i, c) in code.rev().enumerate() {
            if let Some(mut digit) = c.to_digit(10) {
                if i % 2 == 1 {
                    digit = double_digit(digit);
                }
                println!("digit: {}", digit);
                println!("acc: {}", sum);
                println!("sum: {}", sum + digit);
                sum += digit;
            } else {
                return false;
            }
        }
        if sum % 10 == 0 {
            true
        } else {
            false
        }
    }
}
