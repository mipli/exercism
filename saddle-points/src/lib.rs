pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let is_smallest = |val: u64, idx: usize| input.iter().all(|row| { row[idx] >= val });
    input.iter()
        .enumerate()
        .filter_map(|(row_count, row)| {
            match row.iter().max_by_key(|v| *v) {
                Some(max) => {
                    let res = row.iter()
                        .enumerate()
                        .filter_map(|(idx, val)| {
                            if val == max && is_smallest(*val, idx) {
                                Some((row_count as usize, idx as usize))
                            } else {
                                None
                            }
                        }).collect::<Vec<_>>();
                    match res.len() {
                        0 => None,
                        _ => Some(res)
                    }
                },
                None => None
            }
        })
        .flatten()
        .collect::<Vec<_>>()
}
