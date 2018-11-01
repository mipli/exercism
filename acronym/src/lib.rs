pub fn abbreviate(phrase: &str) -> String {
    phrase
        .chars()
        .zip(" ".chars().chain(phrase.chars()))
        .filter_map(|(c, prev)| {
            if prev == ' ' || prev == '-' || c.is_uppercase() && prev.is_lowercase() {
                Some(c.to_uppercase().next()?)
            } else {
                None
            }
        }).collect::<String>()
}
