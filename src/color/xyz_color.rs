use num_traits::Zero;
use std::iter::Sum;
use std::ops::{Add, Div, Mul};

#[derive(Clone, Copy)]
pub struct XYZColor {
    x: f32,
    y: f32,
    z: f32,
}

impl XYZColor {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x: x, y: y, z: z }
    }
}

impl Add for XYZColor {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Mul<f32> for XYZColor {
    type Output = Self;
    fn mul(self, other: f32) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Div<f32> for XYZColor {
    type Output = Self;
    fn div(self, other: f32) -> Self {
        self * (1.0 / other)
    }
}

impl Zero for XYZColor {
    fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }
    fn is_zero(&self) -> bool {
        self.x.is_zero() && self.y.is_zero() && self.z.is_zero()
    }
}

impl Sum for XYZColor {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::zero(), Add::add)
    }
}
