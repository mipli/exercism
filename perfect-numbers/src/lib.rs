#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        return None;
    }
    let sum = (1..=(num/2)).fold(0, |acc, n| {
        if num % n == 0 {
            acc + n
        } else {
            acc
        }
    });
    if sum < num {
        Some(Classification::Deficient)
    } else if sum > num {
        Some(Classification::Abundant)
    } else {
        Some(Classification::Perfect)
    }
}
