use math::Float;

#[derive(Debug, Copy, Clone)]
pub struct HdrColor<F: Float> {
    pub r: F,
    pub g: F,
    pub b: F,
}
