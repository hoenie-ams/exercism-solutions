use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut result = HashSet::new();

    for candidate in possible_anagrams {
        // Skip if the candidate is equal to the word
        if *candidate.to_lowercase() == word.to_lowercase() {
            continue;
        };

        let mut candidate_chars: Vec<char> = candidate.to_lowercase().chars().collect();
        candidate_chars.sort();

        let mut word_chars: Vec<char> = word.to_lowercase().chars().collect();
        word_chars.sort();

        if candidate_chars == word_chars {
            result.insert(*candidate);
        }
    }
    result
}
