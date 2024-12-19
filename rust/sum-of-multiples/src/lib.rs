use std::collections::HashSet;

fn find_factors(limit: u32, factor: &u32) -> Vec<u32> {
    if factor > &0 {
        let mut factors: Vec<u32> = Vec::new();
        for i in 1..=limit - 1 {
            if i % factor == 0 {
                factors.push(i);
            }
        }
        factors
    } else {
        vec![0]
    }
}

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut multiples: Vec<u32> = Vec::new();
    for factor in factors {
        let factor_vec = find_factors(limit, factor);
        println!("For {} found factors {:?}", factor, factor_vec);
        for factor in factor_vec {
            multiples.push(factor);
        }
    }
    // combine into a HashSet using into_iter
    let result_hash: HashSet<u32> = multiples.into_iter().collect();
    result_hash.iter().sum()
}
