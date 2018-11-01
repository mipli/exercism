use unicode_segmentation::UnicodeSegmentation;
use std::iter;

fn get_chunk_size(len: usize) -> usize {
    let n = (len as f64).sqrt().floor() as usize;
    if n * n == len {
        return n
    } else {
        return n + 1
    }
}

pub fn encrypt(input: &str) -> String {
    if input.len() == 0 {
        return String::new();
    }
    let normalized = input
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<String>();
    let chunk_size = get_chunk_size(normalized.len());
    let graphemes = UnicodeSegmentation::graphemes(&normalized[..], true).collect::<Vec<_>>();
    graphemes
        .chunks(chunk_size)
        .fold(vec![], |acc, chunk| {
            if acc.len() == 0 {
                chunk.iter().map(|c| c.to_string()).collect::<Vec<String>>()
            } else {
                let chunk_iter = chunk.iter().chain(iter::repeat(&" "));
                acc.iter()
                    .zip(chunk_iter)
                    .map(|(h, t)| {
                        h.chars().chain(t.chars()).collect::<String>()
                    })
                    .collect::<Vec<_>>()
            }
        })
        .join(" ")
}
