use std::collections::{HashMap};

pub struct School {
    grades: HashMap<u32, Vec<String>>
}

impl School {
    pub fn new() -> School {
        School {
            grades: Default::default()
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        self.grades
            .entry(grade)
            .or_insert(vec![])
            .push(student.to_string());
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut grades = self.grades
            .keys()
            .map(|&k| k)
            .collect::<Vec<_>>();
        grades.sort();
        grades
    }

    pub fn grade(&self, grade: u32) -> Option<Vec<String>> {
        let mut grades = self.grades
            .get(&grade)?
            .to_vec();
        grades.sort();
        Some(grades)
    }
}
