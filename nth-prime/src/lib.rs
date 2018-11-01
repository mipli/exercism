#![feature(nll)]
use std::collections::{HashMap};

pub fn nth(n: u32) -> Option<u64> {
    let mut primes = Primes::new();
    primes.nth(n)
}

struct Primes {
    prime: u64,
    memory: HashMap<u64, u64>
}

impl Primes {
    fn new() -> Self {
        Primes {
            prime: 1,
            memory: HashMap::default()
        }
    }

    fn nth(&mut self, n: u32) -> Option<u64> {
        // manually handle the first 3 cases, so we can only check odd numbers afterwards
        if n == 0 {
            return None;
        } else if n == 1 {
            return Some(2);
        } else if n == 2 {
            return Some(3);
        }
        self.prime = 3;
        self.memory.insert(4, 2);
        self.memory.insert(9, 3);
        for _ in 2..n {
           self.next();
        }
        Some(self.prime)
    }

    fn next(&mut self) {
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
    }
}
