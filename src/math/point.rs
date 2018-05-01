use std::ops::{Add, AddAssign, Sub};

use math::{Float, Vector};

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Point<F: Float> {
    pub x: F,
    pub y: F,
    pub z: F,
}

impl<F: Float> Point<F> {
    pub fn new(x: F, y: F, z: F) -> Self {
        Point { x, y, z }
    }

    pub fn origin() -> Self {
        Point {
            x: F::zero(),
            y: F::zero(),
            z: F::zero(),
        }
    }

    pub fn is_origin(&self) -> bool {
        self.x.is_zero() && self.y.is_zero() && self.z.is_zero()
    }
}

impl<'a, F: Float> AddAssign<Vector<F>> for Point<F> {
    fn add_assign(&mut self, rhs: Vector<F>) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
        self.z = self.z + rhs.z;
    }
}

impl<F: Float> Add<Vector<F>> for Point<F> {
    type Output = Point<F>;
    fn add(mut self, rhs: Vector<F>) -> Point<F> {
        self += rhs;
        self
    }
}

impl<F: Float> Sub<Vector<F>> for Point<F> {
    type Output = Point<F>;
    fn sub(self, rhs: Vector<F>) -> Point<F> {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<F: Float> Sub<Point<F>> for Point<F> {
    type Output = Vector<F>;
    fn sub(self, rhs: Point<F>) -> Vector<F> {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
