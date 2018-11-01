pub fn raindrops(n: usize) -> String {
    analyse(&[(3, "Pling"), (5, "Plang"), (7, "Plong")], n)
}

pub fn analyse(patterns: &[(usize, &str)], n: usize) -> String {
    let answer = patterns.iter().filter(|(i, _)| {
            n % i == 0
        }).map(|&(_, s)| s)
        .collect::<String>();
    if answer.is_empty() {
        n.to_string()
    } else {
        answer
    }
}
