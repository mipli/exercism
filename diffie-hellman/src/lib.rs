extern crate rand;
use rand::{thread_rng, Rng};

pub fn private_key(p: u64) -> u64 {
    let mut primes = Primes::new();
    primes.gen(p as usize);
    match primes.get() {
        Some(p) => *p,
        None => 2
    }
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    modular_power(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    modular_power(b_pub, a, p)
}

fn modular_power(b: u64, exp: u64, m: u64) -> u64 {
    // see: https://en.wikipedia.org/wiki/Modular_exponentiation
    let mut res = 1;
    let mut base = b % m;
    let mut e = exp;
    while e > 0 {
        if e % 2 == 1 {
            res = (res * base) % m;
        }
        e = e >> 1;
        base = (base * base) % m;
    }
    res
}

struct Primes {
    pub primes: Vec<u64>
}

impl Primes {
    fn new() -> Self {
        Primes {
            primes: vec![],
        }
    }

    fn get(&self) -> Option<&u64> {
        if self.primes.len() == 0 {
            return None;
        }

        let mut rng = thread_rng();
        rng.choose(&self.primes)
    }

    fn gen(&mut self, max: usize) {
        let mut sieve = vec![true; max];
        let mut prime = 2;

        while prime < max {
            self.primes.push(prime as u64);
            (prime..max).step_by(prime).for_each(|p| {
                sieve[p] = false;
            });
            prime = sieve.iter()
                .enumerate()
                .skip(prime)
                .find(|(_, p)| **p)
                .map(|(i, _)| i)
                .unwrap_or(max);
        }
    }
}
