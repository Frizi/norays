use math::{Float, Point};
use num_traits::Zero;
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector<F: Float> {
    pub x: F,
    pub y: F,
    pub z: F,
}

impl<F: Float> Vector<F> {
    pub fn new(x: F, y: F, z: F) -> Self {
        Vector { x, y, z }
    }

    pub fn plus_x() -> Self {
        Self::new(F::one(), F::zero(), F::zero())
    }

    pub fn plus_y() -> Self {
        Self::new(F::zero(), F::one(), F::zero())
    }

    pub fn plus_z() -> Self {
        Self::new(F::zero(), F::zero(), F::one())
    }

    pub fn recip(self) -> Self {
        Self {
            x: self.x.recip(),
            y: self.y.recip(),
            z: self.z.recip(),
        }
    }

    pub fn dot(self, rhs: Self) -> F {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn magnitude_sq(self) -> F {
        self.dot(self)
    }

    pub fn magnitude(self) -> F {
        self.magnitude_sq().sqrt()
    }

    pub fn normalized(self) -> Self {
        self * self.magnitude().recip()
    }

    pub fn cross(self, rhs: Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    pub fn into_point(self) -> Point<F> {
        Point {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

impl<F: Float> Add<Vector<F>> for Vector<F> {
    type Output = Vector<F>;
    fn add(self, rhs: Vector<F>) -> Vector<F> {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<F: Float> Sub<Vector<F>> for Vector<F> {
    type Output = Vector<F>;
    fn sub(self, rhs: Vector<F>) -> Vector<F> {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<F: Float> Mul<F> for Vector<F> {
    type Output = Vector<F>;
    fn mul(self, rhs: F) -> Vector<F> {
        Vector {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl<F: Float> Zero for Vector<F> {
    fn zero() -> Self {
        Vector {
            x: F::zero(),
            y: F::zero(),
            z: F::zero(),
        }
    }

    fn is_zero(&self) -> bool {
        self.x.is_zero() && self.y.is_zero() && self.z.is_zero()
    }
}
