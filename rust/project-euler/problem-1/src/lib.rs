/// Finds the sum of all multiples of 3 and 5 within a given range.
///
/// [lower_bound, upper_bound)
/// 
/// ## Examples
/// 
/// ```
/// use problem_1::find_sum_of_mutliples_of_3_and_5;
/// 
/// let result = find_sum_of_mutliples_of_3_and_5(0, 1000);
/// assert_eq!(result, 233168);
/// ```
pub fn find_sum_of_mutliples_of_3_and_5(lower_bound: u64, upper_bound: u64) -> u64 {
    let mut sum: u64 = 0;
    for i in lower_bound..=upper_bound - 1 {
        if i % 3 == 0 {
            sum += i;
        } else if i % 5 == 0 {
            sum += i;
        }
    }
    sum 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_ten_natural_numbers() {
        let result = find_sum_of_mutliples_of_3_and_5(0, 10);
        assert_eq!(result, 23);
    }

    #[test]
    fn euler_problem_1() {
        let result = find_sum_of_mutliples_of_3_and_5(0, 1000);
        assert_eq!(result, 233168);
    }
}
