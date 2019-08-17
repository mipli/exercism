use std::cmp::{Ordering};
pub fn find<T: Ord, R: AsRef<[T]>>(array: R, key: T) -> Option<usize> {
    let mut slice = array.as_ref();

    if slice.len() == 0 {
        return None;
    }

    let mut offset = 0;
    loop {
        let idx = slice.len() / 2;
        match slice[idx].cmp(&key) {
            Ordering::Equal => {
                return Some(idx + offset);
            },
            Ordering::Less if slice.len() > 1 => {
                slice = &slice[idx..];
                offset += idx;
            },
            Ordering::Greater if slice.len() > 1 => {
                slice = &slice[..idx];
            },
            _ => {
                return None;
            }
        }
    }
}
