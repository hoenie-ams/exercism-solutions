use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word_lowercase = word.to_lowercase();
    let word_sorted = sort_str(&word_lowercase);

    possible_anagrams
        .iter()
        .filter(|&candidate| {
            let candidate_lower = candidate.to_lowercase();
            let candidate_sorted = sort_str(&candidate_lower);

            candidate_lower != word_lowercase && candidate_sorted == word_sorted
        })
        .copied()
        .collect()
}

pub fn sort_str(word: &str) -> Vec<char> {
    let mut chars: Vec<char> = word.chars().collect();
    chars.sort_unstable();
    chars
}
