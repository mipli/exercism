use std::collections::HashMap;

fn validate_nucleotide(c: char)-> Result<char, char> {
    if ['A', 'C', 'G', 'T'].contains(&c) {
        Ok(c)
    } else {
        Err(c)
    }
}


fn validate_dna(dna: &str) -> Result<&str, char> {
    for c in dna.chars() {
        let _ = validate_nucleotide(c)?;
    }
    Ok(dna)
}

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    let _ = validate_dna(dna)?;
    let _ = validate_nucleotide(nucleotide)?;
    Ok(dna
        .chars()
        .filter(|c| *c == nucleotide)
        .count())
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let _ = validate_dna(dna)?;
    let mut counts: HashMap<char, usize> = HashMap::default();
    counts.insert('A', 0);
    counts.insert('T', 0);
    counts.insert('C', 0);
    counts.insert('G', 0);
    dna.chars().for_each(|c| {
        counts
            .entry(c)
            .and_modify(|n| *n += 1);
    });
    Ok(counts)
}
