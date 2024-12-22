pub fn nth(n: u32) -> u32 {
    let mut index_array = vec![true; (n * n + 10) as usize];
    // 0 and 1 are not prime
    index_array[0] = false;
    index_array[1] = false;

    // Find the upper bound
    let end_index = ((index_array.len() as f64).sqrt().ceil()) as usize;

    // Sieve of Eratosthenes to mark non-prime numbers
    for i in 2..end_index {
        if index_array[i] {
            let mut j = i * i;
            while j < index_array.len() {
                index_array[j] = false;
                j += i;
            }
        }
    }

    // Find the nth prime (zero-based)
    let mut prime_count = 0;
    for (index, &is_prime) in index_array.iter().enumerate() {
        if is_prime {
            if prime_count == n {
                return index as u32;
            }
            prime_count += 1;
        }
    }

    panic!("Unable to find the nth prime"); // Fallback in case of an error
}
