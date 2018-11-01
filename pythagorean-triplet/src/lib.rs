fn is_triplet(a: u32, b: u32, c: u32) -> bool {
    a.pow(2) + b.pow(2) == c.pow(2)
}

pub fn find() -> Option<u32> {
    let triplet = (1..998).filter_map(|a| {
        let rem = 999 - a;
        match (1..rem).find(|b| is_triplet(a, *b, 1_000 - a - b)) {
            Some(b) => Some((a, b)),
            None => None
        }
    }).nth(0);
    match triplet {
        Some((a, b)) => Some(a * b * (1_000 - a - b)),
        None => None
    }
}
