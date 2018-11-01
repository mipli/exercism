use num::{FromPrimitive, Num};

pub struct Triangle<T: Copy + PartialOrd + Num + FromPrimitive> {
    a: T,
    b: T,
    c: T
}

impl<T: Copy + PartialOrd + Num + FromPrimitive> Triangle<T> {
    pub fn build(sides: [T; 3]) -> Option<Triangle<T>> {
        match sides {
            [a, b, c] if (a == T::zero() || b == T::zero() || c == T::zero()) => None,
            [a, b, c] if (a + b < c) || (b + c < a || a + c < b) => None,
            [a, b, c] => Some(Triangle {a, b, c})
        }
    }

    pub fn is_equilateral(&self) -> bool {
        self.a == self.b && self.b == self.c
    }

    pub fn is_scalene(&self) -> bool {
        self.a != self.b && self.b != self.c && self.c != self.a
    }

    pub fn is_isosceles(&self) -> bool {
        self.a == self.b || self.a == self.c || self.b == self.c
    }
}
