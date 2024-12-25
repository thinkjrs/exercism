pub fn collatz(n: u64) -> Option<u64> {
    match n {
        0 => None,
        1 => Some(0),
        _ => {
            let mut num_steps = 0;
            let mut n = n;

            while n != 1 {
                n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
                num_steps += 1;
            }
            Some(num_steps)
        }
    }
}
