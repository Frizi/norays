use math::Float;

#[derive(Clone, Copy)]
pub struct UV<F> {
    pub u: F,
    pub v: F,
}

impl<F: Float> Default for UV<F> {
    fn default() -> Self {
        Self {
            u: F::zero(),
            v: F::zero(),
        }
    }
}
