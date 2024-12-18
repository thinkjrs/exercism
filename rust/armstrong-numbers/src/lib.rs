pub fn is_armstrong_number(num: u32) -> bool {
    //todo!("true if {num} is an armstrong number")
    // convert num to string
    let num_str = num.to_string();
    // get the number of digits
    let num_digits = num_str.len();
    // iterate through each digit
    let num_iter = num_str.chars().map(|c| c.to_digit(10).unwrap());
    // raise each digit to the power of the number of digits
    let sum: u32 = num_iter.map(|d| d.pow(num_digits as u32)).sum();
    // sum the results
    sum == num
}
