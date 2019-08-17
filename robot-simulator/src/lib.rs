#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

use std::ops::{Add};
impl Add<&Direction> for (i32, i32) {
    type Output = (i32, i32);

    fn add(self, other: &Direction) -> (i32, i32) {
        match other {
            Direction::North => (self.0, self.1 + 1),
            Direction::East => (self.0 + 1, self.1),
            Direction::South => (self.0, self.1 - 1),
            Direction::West => (self.0 - 1, self.1)
        }
    }
}

pub struct Robot {
    pos: (i32, i32),
    facing: Direction
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot {
            pos: (x, y),
            facing: d
        }
    }

    pub fn turn_right(self) -> Self {
        let facing = match self.facing {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North
        };
        Robot {
            facing,
            ..self
        }
    }

    pub fn turn_left(self) -> Self {
        let facing = match self.facing {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South
        };
        Robot {
            facing,
            ..self
        }
    }

    pub fn advance(self) -> Self {
        Robot {
            pos: self.pos + &self.facing,
            ..self
        }
    }

    pub fn instructions(self, instructions: &str) -> Self {
        instructions.chars().fold(self, |robot, c| {
            match c {
                'A' => robot.advance(),
                'L' => robot.turn_left(),
                'R' => robot.turn_right(),
                _ => robot
            }
        })
    }

    pub fn position(&self) -> (i32, i32) {
        self.pos
    }

    pub fn direction(&self) -> &Direction {
        &self.facing
    }
}
