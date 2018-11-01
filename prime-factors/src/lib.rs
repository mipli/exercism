#![feature(nll)]
use std::collections::{HashMap};

pub fn factors(n: u64) -> Vec<u64> {
    let mut rem = n;
    let primes = Primes::new();
    let mut factors = vec![];
    for prime in primes {
        while rem % prime == 0 { 
            factors.push(prime);
            rem = rem / prime;
        }
        if rem == 1 {
            break;
        }
    }
    factors
}

struct Primes {
    prime: u64,
    memory: HashMap<u64, u64>
}

impl Primes {
    fn new() -> Self {
        let mut memory = HashMap::default();
        memory.insert(4, 2);
        memory.insert(9, 3);
        Primes {
            prime: 1,
            memory
        }
    }
}

impl Iterator for Primes {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        if self.prime == 1 {
            self.prime = 2;
            return Some(2);
        } else if self.prime == 2 {
            self.prime = 3;
            return Some(3);
        }
        let mut found = false;
        let mut num = self.prime + 2;
        while !found {
            let q = num;
            num += 2;
            match self.memory.get(&q) {
                Some(p) => {
                    let mut x = p + q;
                    while x % 2 == 0 || self.memory.contains_key(&x) {
                        x += p;
                    }
                    self.memory.insert(x, *p);
                },
                None => {
                    found = true;
                    self.prime = q;
                    self.memory.insert(self.prime * self.prime, self.prime);
                }
            };
        }
        Some(self.prime)
    }
}
