#[derive(Debug, PartialEq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char)
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    if span == 0 {
        return Ok(1);
    }
    let nums = string_digits.chars()
        .try_fold(vec![], |mut acc, c| {
            match c.to_digit(10) {
                Some(n) => {
                    acc.push(n as u64);
                    Ok(acc)
                },
                None => Err(Error::InvalidDigit(c))
            }
        })?;

    nums
        .windows(span)
        .map(|ns| ns.iter().product())
        .max()
        .ok_or(Error::SpanTooLong)
}
