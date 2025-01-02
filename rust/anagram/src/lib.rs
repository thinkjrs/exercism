use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut result: HashSet<&'a str> = HashSet::new();

    let lowercase_word = word.to_lowercase();

    // loop through possible_anagrams
    for &anagram in possible_anagrams {
        let lowercase_anagram = anagram.to_lowercase();

        // given each possible anagram, ask if all characters in word are present in anagram
        if lowercase_anagram != lowercase_word && lowercase_anagram.len() == lowercase_word.len() {
            let mut word_chars = lowercase_word.clone();
            let is_anagram = lowercase_anagram.chars().all(|c| {
                if let Some(pos) = word_chars.find(c) {
                    word_chars.remove(pos);
                    return true;
                }
                false
            });

            if is_anagram {
                result.insert(anagram);
            }
        }
    }
    // if yes, store in HashSet
    result
}
