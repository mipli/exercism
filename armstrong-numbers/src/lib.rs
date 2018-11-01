trait WithDigits {
    fn digits(&self) -> Vec<u32>;
}

impl WithDigits for u32 {
    fn digits(&self) -> Vec<u32> {
        fn inner(n: u32, xs: &mut Vec<u32>) {
            if n >= 10 {
                inner(n / 10, xs);
            }
            xs.push(n % 10);
        }
        let mut xs = Vec::new();
        inner(*self, &mut xs);
        xs
    }
}

pub fn is_armstrong_number(num: u32) -> bool {
    let nums = num.digits();
    let len = nums.len() as u32;
    nums.iter().fold(0, |acc, n| acc + n.pow(len)) == num
}
