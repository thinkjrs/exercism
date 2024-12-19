use std::collections::HashSet;

fn find_factors(limit: u32, factor: &u32) -> Vec<u32> {
    if factor == &0 {
        return vec![0];
    }

    (1..limit).filter(|&i| i % factor == 0).collect()
}

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    factors
        .iter()
        .flat_map(|factor| find_factors(limit, factor))
        .collect::<HashSet<u32>>()
        .iter()
        .sum::<u32>()
}
