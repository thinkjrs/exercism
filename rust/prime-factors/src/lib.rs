pub fn factors(n: u64) -> Vec<u64> {
    let mut factors = vec![];
    let mut n = n;
    while n % 2 == 0 {
        factors.push(2);
        n /= 2;
    }

    for i in (3..=(n as f64).sqrt() as u64).step_by(2) {
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
