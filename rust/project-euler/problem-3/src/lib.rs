pub fn prime_factors(n: u64) -> Vec<u64> {
    let mut factors: Vec<u64> = Vec::new();
    let mut n = n;

    while n % 2 == 0 {
        factors.push(2);
        n /= 2;
    }
    for i in (3..((n as f64).sqrt() as u64)).step_by(2) {
        while n % i == 0 {
            factors.push(i);
            n /= i;
        }
    }

    if n > 2 {
        factors.push(n);
    }

    factors
}

pub fn biggest_prime_factor(n: u64) -> u64 {
    let mut n = n;
    let mut last_factor = 0;
    while n % 2 == 0 {
        n /= 2;
        last_factor = 2;
    }
    for i in (3..((n as f64).sqrt() as u64)).step_by(2) {
        while n % i == 0 {
            last_factor = i;
            n /= i;
        }
    }

    if n > 2 {
        if n > last_factor {
            return n;
        };
    }

    last_factor
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prime_factors_of_2() {
        let result = prime_factors(2);
        assert_eq!(result, vec![2]);
    }
    #[test]
    fn largest_prime_factor_of_13195() {
        let result = biggest_prime_factor(13195);
        assert_eq!(result, 29);
    }
}
