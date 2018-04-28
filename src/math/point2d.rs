use math::Float;

#[derive(Clone)]
pub struct Point2D<F: Float> {
    pub x: F,
    pub y: F,
}

impl<F: Float> Point2D<F> {
    pub fn new(x: F, y: F) -> Self {
        Point2D { x, y }
    }
}
