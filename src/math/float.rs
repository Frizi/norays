use num_traits::{Float as NTFloat, FloatConst};
use std::fmt::Debug;

pub trait Float: Debug + NTFloat + FloatConst + Send + Sync + 'static {
    fn lerp(self, a: Self, b: Self) -> Self {
        a + self * (b - a)
    }
}

impl Float for f32 {}
impl Float for f64 {}
