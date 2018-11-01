#![feature(test)]
extern crate test;

pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    if upper_bound < 2 {
        // special case for 2, so we can step by two when checking for primes
        return vec![];
    }
    let mut primes = vec![2];
    let mut sieve = seed_sieve(upper_bound + 1);
    let mut prime = 3;

    while prime <= upper_bound {
        primes.push(prime);
        (prime..=upper_bound).step_by(prime as usize).for_each(|p| {
            sieve[p as usize].1 = false;
        });
        prime = match next_prime(prime, &sieve) {
            Some(p) => p,
            None => break
        }
    }
    primes
}

fn next_prime(prime: u64, sieve: &Vec<(u64, bool)>) -> Option<u64> {
    sieve.iter()
        .skip(prime as usize)
        .step_by(2)
        .find(|(_, p)| *p)
        .map(|(i, _)| *i)
}

fn seed_sieve(limit: u64) -> Vec<(u64, bool)> {
    (0..limit).map(|i| (i as u64, true)).collect()
}

pub fn primes_up_to_retain(upper_bound: u64) -> Vec<u64> {
    let mut primes = vec![];
    let mut sieve = (2..=upper_bound).rev().collect::<Vec<_>>();
    while let Some(prime) = sieve.pop() {
        primes.push(prime);
        sieve.retain(|s| s % prime != 0);
    }
    primes
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    // benched to 48,000 ns/iter
    #[bench]
    fn bench_ten_thousand_limit_simple(b: &mut Bencher) {
        b.iter(|| primes_up_to(10000))
    }

    // benched to 6,000,000 ns/iter
    #[bench]
    fn bench_ten_thousand_limit_retain(b: &mut Bencher) {
        b.iter(|| primes_up_to_retain(10000))
    }
}
