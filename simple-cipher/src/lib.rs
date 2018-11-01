use rand::{Rng, thread_rng};

fn is_valid_key(key: &str) -> bool {
    key.len() > 0 && key.chars().all(|c| c.is_ascii_lowercase())
}
pub fn encode(key: &str, s: &str) -> Option<String> {
    if !is_valid_key(key) {
        return None;
    }
    let enc = s.chars().zip(key.chars().cycle()).map(|(c, k)| {
        let offset = k as u8 - b'a';
        (((c as u8 - b'a' + offset) % 26) + b'a') as char
    }).collect::<String>();
    Some(enc)
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    if !is_valid_key(key) {
        return None;
    }
    let enc = s.chars().zip(key.chars().cycle()).map(|(c, k)| {
        let offset = ((26i8 - ((k as u8 - b'a') as i8)) % 26) as u8;
        (((c as u8 - b'a' + offset) % 26) + b'a') as char
    }).collect::<String>();
    Some(enc)
}

pub fn encode_random(s: &str) -> (String, String) {
    let mut rng = thread_rng();
    let key = (0..100).map(|_| {
        (rng.gen_range(0, 26) + b'a') as char
    }).collect::<String>();
    let encoded = encode(&key, s).unwrap();
    (key, encoded)
}
