pub fn build_proverb(list: &[&str]) -> String {
    let mut result = String::new();
    for (i, item) in list.iter().enumerate() {
        if i < list.len() - 1 {
            result.push_str(&format!("For want of a {} ", item));
            result.push_str(&format!("the {} was lost.\n", list[i + 1]));
        } else {
            result.push_str(&format!("And all for the want of a {}.", list[0]));
        }
    }
    result
}
