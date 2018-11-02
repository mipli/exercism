#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base <= 1 {
        return Err(Error::InvalidInputBase);
    } else if to_base <= 1 {
        return Err(Error::InvalidOutputBase);
    }
    let mut lab = number
        .iter()
        .rev()
        .enumerate()
        .try_fold(0, |acc, (i, n)| {
            if *n >= from_base {
                Err(Error::InvalidDigit(*n))
            } else {
                Ok(acc + (n * from_base.pow(i as u32))) 
            }
        })?;
    let mut out = vec![];
    while lab > 0 {
        out.push(lab % to_base);
        lab = lab / to_base;
    }
    out.reverse();
    Ok(out)
}
