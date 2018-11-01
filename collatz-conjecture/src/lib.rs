pub fn collatz(n: u64) -> Option<u64> {
    if n <= 0 {
        return None;
    }
    let mut rem = n;
    let mut steps = 0;
    while rem != 1 {
        steps += 1;
        rem = match rem % 2 {
            0 => rem / 2,
            1 => (3 * rem) + 1,
            _ => panic!("n % 2 should not give this result")
        };
    }
    Some(steps)
}
