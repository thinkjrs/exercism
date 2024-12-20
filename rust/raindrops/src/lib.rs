use std::collections::HashMap;

pub fn raindrops(n: u32) -> String {
    let closure_vec: Vec<Box<dyn Fn(u32) -> bool>> = vec![
        Box::new(|n: u32| n % 3 == 0),
        Box::new(|n: u32| n % 5 == 0),
        Box::new(|n: u32| n % 7 == 0),
    ];
    let mut words: HashMap<u32, String> = HashMap::new();
    words.insert(0, String::from("Pling"));
    words.insert(1, String::from("Plang"));
    words.insert(2, String::from("Plong"));
    let mut result = String::new();
    for (index, closure) in closure_vec.iter().enumerate() {
        if closure(n) {
            if let Some(word) = words.get(&(index as u32)) {
                result += word;
            }
        }
    }
    if result.len() == 0 {
        result = format!("{}", n).to_string();
    }
    result
}
