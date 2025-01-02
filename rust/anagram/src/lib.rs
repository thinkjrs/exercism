use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut result: HashSet<&'a str> = HashSet::new();
    // loop through possible_anagrams
    for anagram in possible_anagrams {
        // given each possible anagram, ask if all characters in word are present in anagram
        if anagram.to_lowercase() != word.to_lowercase() && anagram.len() == word.len() {
            let mut word = word.to_string().to_lowercase();
            let test = anagram
                .chars()
                .filter(|&c| {
                    let lower_c = c.to_lowercase().next().unwrap();
                    if word.contains(lower_c) {
                        if let Some(pos) = word.find(lower_c) {
                            word.remove(pos);
                            return true;
                        }
                    }
                    false
                })
                .collect::<String>();
            if test.as_str().to_lowercase() == *anagram.to_lowercase() {
                result.insert(*anagram);
            }
        }
    }
    // if yes, store in HashSet
    result
}
