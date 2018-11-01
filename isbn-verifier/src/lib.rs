fn to_digits(isbn: &str) -> Result<Vec<u32>, ()> {
    isbn.chars().rev().enumerate().try_fold(vec![], |mut acc, (i, c)| {
        match c.to_digit(10) {
            Some(v) => {
                acc.push(v);
                Ok(acc)
            },
            None if i == 0 && c == 'X' => {
                acc.push(10);
                Ok(acc)
            },
            None if c == '-' => Ok(acc),
            None => Err(())
        }
    })
}
pub fn is_valid_isbn(isbn: &str) -> bool {
    match to_digits(isbn) {
        Ok(ref nums) if nums.len() == 10 => {
            nums.iter().enumerate().fold(0, |acc, (i, n)| {
                acc + (n * ((i as u32) + 1))
            }) % 11 == 0
        },
        _ => false
    }
}
