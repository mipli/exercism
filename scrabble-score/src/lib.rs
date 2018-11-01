use std::collections::{HashMap};
use lazy_static::*;

lazy_static! {
    static ref POINTS: HashMap<char, u64> = {
        let mut points = HashMap::default();
        points.insert('a', 1);
        points.insert('b', 3);
        points.insert('c', 3);
        points.insert('d', 2);
        points.insert('e', 1);
        points.insert('f', 4);
        points.insert('g', 2);
        points.insert('h', 4);
        points.insert('i', 1);
        points.insert('j', 8);
        points.insert('k', 5);
        points.insert('l', 1);
        points.insert('m', 3);
        points.insert('n', 1);
        points.insert('o', 1);
        points.insert('p', 3);
        points.insert('q', 10);
        points.insert('r', 1);
        points.insert('s', 1);
        points.insert('t', 1);
        points.insert('u', 1);
        points.insert('v', 4);
        points.insert('w', 4);
        points.insert('x', 8);
        points.insert('y', 4);
        points.insert('z', 10);
        points
    };
}

pub fn score(word: &str) -> u64 {
    word.to_lowercase().chars()
        .fold(0, |acc, c| acc + get_points(&c))
}

fn get_points(c: &char) -> &u64 {
    POINTS.get(c).unwrap_or(&0)
}
