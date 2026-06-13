use std::collections::HashSet;

fn normalize(word: &str) -> Vec<char> {
    let mut output: Vec<char> = word.to_lowercase().chars().collect();
    output.sort_unstable();
    output
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    possible_anagrams
        .iter()
        .filter(|anagram| {
            let is_not_same_word = anagram.to_lowercase() != word.to_lowercase();
            let is_equal_normalized = normalize(anagram) == normalize(word);
            is_not_same_word && is_equal_normalized
        })
        .map(|&a| a)
        .collect()
}
