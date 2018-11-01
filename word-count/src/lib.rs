use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    words.chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace())
        .collect::<String>()
        .to_lowercase()
        .split_whitespace()
        .fold(HashMap::new(), |mut counts, word| {
            *counts.entry(word.to_string()).or_insert(0) += 1;
            counts
        })
}
