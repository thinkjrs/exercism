pub fn square(s: u32) -> u64 {
    if s < 65 {
        sum_doubles(s as u64)
    } else {
        panic!(
            "There are not more than 64 squares on the board. You chose {}, try again.",
            s
        );
    }
}

pub fn total() -> u64 {
    sum_all_doubles(64 as u64)
}

pub fn sum_all_doubles(n: u64) -> u64 {
    let sum: u64 = (1..n + 1).map(|num| sum_doubles(num)).sum();
    sum
}

pub fn sum_doubles(n: u64) -> u64 {
    let double: u64 = (2 as u64).pow(n as u32 - 1);
    double
}
