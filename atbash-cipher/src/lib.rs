/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    decode(plain)
        .chars()
        .fold(String::new(), |mut acc, c| {
            if acc.len() % 6 == 5 {
                acc = acc + " ";
            }
            acc + &c.to_string()
        })
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    let mut v = [0; 1];
    cipher.to_lowercase()
        .chars()
        .filter(|c| c.is_ascii() && c.is_alphanumeric())
        .map(|c| {
            if c.is_numeric() {
                c
            } else {
                c.encode_utf8(&mut v);
                (219 - v[0]) as char
            }
        })
        .collect::<String>()
}
