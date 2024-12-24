pub fn factors(n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut n = n;

    while n % 2 == 0 {
        factors.push(2);
        n /= 2;
    }
    let max_factor_limit = (n as f64).sqrt() as u64;
    let range = (3..=max_factor_limit).step_by(2);

    factors.extend(range.flat_map(|i| {
        let mut tmp_factors = vec![0u64; max_factor_limit as usize];
        let mut count = 0;

        while n % i == 0 {
            tmp_factors[count] = i;
            count += 1;
            n /= i;
        }
        tmp_factors[..count].to_vec()
    }));

    if n > 2 {
        factors.push(n);
    }

    factors
}
