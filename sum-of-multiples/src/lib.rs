#![feature(test)]
extern crate test;

use std::collections::{HashSet};

fn get_multiples(limit: u32, factor: u32) -> Vec<u32> {
    (factor..).step_by(factor as usize).take_while(|n| n < &limit).collect()
}

pub fn simple(limit: u32, factors: &[u32]) -> u32 {
    let mut multiples = factors
        .iter()
        .flat_map(|&factor| get_multiples(limit, factor))
        .collect::<Vec<_>>();
    multiples.sort();
    multiples.dedup();
    multiples.iter().sum()
}

pub fn hashset(limit: u32, factors: &[u32]) -> u32 {
    let set: HashSet<u32> = factors
        .iter()
        .flat_map(|&factor| get_multiples(limit, factor))
        .collect();
    set.iter().sum()
}

pub fn brute_force(limit: u32, factors: &[u32]) -> u32 {
    (1..limit).filter(|n| {
        factors.iter().any(|f| n % f == 0)
    }).fold(0, |acc, i| acc + i)
}
pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    // hashset(limit, factors)
    // brute_force(limit, factors)
    simple(limit, factors)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    // 864_000 ns/iter
    #[bench]
    fn bench_simple(b: &mut Bencher) {
        b.iter(|| simple(100000, &[3, 5, 13, 43, 47]));
    }

    // 944_000 ns/iter
    #[bench]
    fn bench_brute_force(b: &mut Bencher) {
        b.iter(|| brute_force(100000, &[3, 5, 13, 43, 47]));
    }

    // 4_917_000 ns/iter
    #[bench]
    fn bench_hashset(b: &mut Bencher) {
        b.iter(|| hashset(100000, &[3, 5, 13, 43, 47]));
    }
}
