pub fn fibonacci_sequence_max_value(number_size_upper_bound: u64) -> Vec<u64> {
    let mut prev: u64 = 0;
    let mut curr: u64 = 1;
    let mut storage: Vec<u64> = vec![0, 1];

    loop {
        let temp = curr;
        curr = curr + prev;
        prev = temp;
        if curr < number_size_upper_bound {
            storage.push(curr);
            continue;
        }
        break;
    }
    storage
}
pub fn fibonacci_sequence(n: u64) -> Vec<u64> {
    match n {
        0 => vec![0],
        1 => vec![0, 1],
        _ => {
            let mut prev: u64 = 0;
            let mut curr: u64 = 1;
            let mut storage: Vec<u64> = vec![0, 1];
            for _ in 2..n {
                let temp = curr;
                curr = curr + prev;
                prev = temp;
                storage.push(curr);
            }
            storage
        }
    }
}

pub fn sum_even_fibonacci(sequence: &Vec<u64>, lower_bound: u64, upper_bound: u64) -> u64 {
    let lower_bound = if lower_bound > 0 {
        lower_bound as usize - 1
    } else {
        0
    };

    sequence[lower_bound..upper_bound as usize]
        .iter()
        .filter(|&s| s % 2 == 0)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_first_6() {
        let result = fibonacci_sequence(6);
        assert_eq!(result, vec![0, 1, 1, 2, 3, 5,]);
    }
    #[test]
    fn it_works_first_12() {
        let result = fibonacci_sequence(12);
        assert_eq!(result, vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89]);
    }
    #[test]
    fn summation_works_first_5() {
        let even_sum: u64 = 2;
        let sequence = fibonacci_sequence(1 + 6);
        let result = sum_even_fibonacci(&sequence, 1, 6);
        assert_eq!(result, even_sum);
    }
    #[test]
    fn summation_works_five_after_2() {
        let even_sum: u64 = 10;
        let sequence = fibonacci_sequence(2 + 7);

        let result = sum_even_fibonacci(&sequence, 2, 7);

        assert_eq!(result, even_sum);
    }

    #[test]
    fn it_works_max_value_sequence_first_12() {
        let result = fibonacci_sequence_max_value(90);
        assert_eq!(result, vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89]);
    }
    #[test]
    fn summation_of_evens_works_max_value_90() {
        let sequence = fibonacci_sequence_max_value(13);
        let upper_bound: u64 = sequence
            .len()
            .try_into()
            .expect("The sequence length didn't fit into the u64");
        let result = sum_even_fibonacci(&sequence, 0, upper_bound);
        assert_eq!(result, 10);
    }
}
