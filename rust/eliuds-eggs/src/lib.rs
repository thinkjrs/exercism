pub fn egg_count(display_value: u32) -> usize {
    //todo!("count the eggs in {display_value}")
    let display_value_binary_chars: Vec<char> = format!("{:b}", display_value).chars().collect();
    let num_eggs: usize = display_value_binary_chars
        .iter()
        .filter(|&c| *c == '1')
        .count();

    num_eggs
}
