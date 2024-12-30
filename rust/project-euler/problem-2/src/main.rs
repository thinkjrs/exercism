use problem_2::{fibonacci_sequence_max_value, sum_even_fibonacci};

fn main() {
    let max_value: u64 = 4_000_000;
    let sequence = fibonacci_sequence_max_value(max_value);
    let upper_bound: u64 = sequence
        .len()
        .try_into()
        .expect("The sequence length didn't fit into the u64");

    let result = sum_even_fibonacci(&sequence, 0, upper_bound);
    println!("Max value {} sum: {}", max_value, result);
}
