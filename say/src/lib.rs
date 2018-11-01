use lazy_static::*;

lazy_static! {
    static ref ONE: Vec<&'static str> = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen", "eighteen", "nineteen"];
    static ref TWEENS: Vec<&'static str> = vec!["twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety"];
    static ref SCALE: Vec<Option<&'static str>> = vec![None, Some("thousand"), Some("million"), Some("billion"), Some("trillion"), Some("quadrillion"), Some("quintillion")];
}

fn first_digit(n: u64) -> u64 {
    let mut n = n;
    while n > 9 {
        n /= 10;
    }
    n
}

fn decompose_hundreds(n: u64) -> String {
    let x = first_digit(n);

    match n {
        0 => "".to_string(),
        1...19 => ONE[(n - 1) as usize].to_string(),
        20...99 if n % 10 == 0 => TWEENS[(x - 2) as usize].to_string(),
        20...99 => TWEENS[(x - 2) as usize].to_string() + "-" + &decompose_hundreds(n - x * 10),
        100...1000 if n % 100 == 0 => ONE[(x - 1) as usize].to_string() + " hundred",
        100...1000 => ONE[(x - 1) as usize].to_string() + " hundred " + &decompose_hundreds(n - x*100),
        _ => "".to_string()
    }
}

fn decompose(n: u64) -> String {
    thousand_split(n).iter().zip(SCALE.iter()).rev().filter_map(|(num, sc)| {
        if *num == 0 {
            return None;
        }
        let hundreds = decompose_hundreds(*num);
        match sc {
            Some(scale) => {
                Some(hundreds + " " + scale)
            },
            None => {
                Some(hundreds)
            }
        }
    }).collect::<Vec<_>>().join(" ")
}

fn thousand_split(n: u64) -> Vec<u64> {
    let mut n = n;
    let mut splits = vec![];
    while n >= 1000 {
        let t = n % 1000;
        splits.push(t);
        n = (n - t) / 1000;
    }
    splits.push(n);
    splits
}

pub fn encode(n: u64) -> String {
    if n == 0 {
        "zero".to_string()
    } else {
        decompose(n)
    }
}
