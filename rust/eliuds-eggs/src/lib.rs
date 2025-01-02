pub fn egg_count(display_value: u32) -> usize {
    let num_eggs: usize = format!("{:b}", display_value)
        .chars()
        .filter(|c| *c == '1')
        .count();

    num_eggs
}
