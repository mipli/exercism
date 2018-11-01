pub fn rotate(input: &str, key: u8) -> String {
    fn calc(val: u8, offset: u8, key: u8) -> char {
        (((val - offset + key) % 26) + offset) as char
    }
    input.chars().map(|c| {
        match c {
            'a'...'z' => calc(c as u8, b'a', key),
            'A'...'Z' => calc(c as u8, b'A', key),
            _ => c
        }
    })
    .collect::<String>()
}
