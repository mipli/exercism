/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    match convert(code) {
        Err(_) => false,
        Ok(ref nums) if nums.len() <= 1 => false,
        Ok(nums) => {
            nums.iter()
                .rev()
                .enumerate()
                .map(|(i, &c)| {
                    if i % 2 == 1 {
                        if c > 4 {
                            (c * 2) - 9
                        } else {
                            c * 2
                        }
                    } else {
                        c
                    }
                })
                .sum::<u32>() % 10 == 0
        }
    }
}

fn convert(code: &str) -> Result<Vec<u32>, ()> {
    code.chars().try_fold(vec![], |mut acc, c| {
        match c.to_digit(10) {
            Some(d) => {
                acc.push(d);
                Ok(acc)
            },
            None if c == ' ' => Ok(acc),
            None => Err(())
        }
    })
}

