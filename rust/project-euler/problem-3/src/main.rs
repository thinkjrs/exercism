use problem_3::biggest_prime_factor;
use std::env;

fn main() {
    //let number_to_factorize = env::args().nth(1).unwrap().parse::<u64>().unwrap();
    let number_to_factorize = env::args()
        .nth(1)
        .unwrap_or("600851475143".parse().unwrap())
        .parse::<u64>()
        .unwrap();
    println!(
        "Largest prime factor of {} is {}",
        number_to_factorize,
        biggest_prime_factor(number_to_factorize)
    );
}
