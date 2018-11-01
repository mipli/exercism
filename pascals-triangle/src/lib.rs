pub struct PascalsTriangle {
    rows: Vec<Vec<u32>>
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let rows = (0..row_count).map(|row_idx| {
            (0..=row_idx).map(|idx| binomial(row_idx, idx)).collect::<Vec<_>>()
        }).collect::<Vec<_>>();
        PascalsTriangle {
            rows
        }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.rows.clone()
    }
}

fn binomial(n: u32, k: u32) -> u32 {
    let fact = |n| (1..=n).fold(1, |acc, i| acc * i);
    fact(n)/(fact(k) * fact(n - k))
}
