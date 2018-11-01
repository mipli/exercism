use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    h.iter().flat_map(|(&k, v)| {
        v.iter().map(move |c| {
            (c.to_lowercase().next().expect("Could not convert char to lowercase"), k)
        })
    }).collect()
}
