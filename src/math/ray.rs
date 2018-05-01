use math::{Float, Point, Vector};

#[derive(Debug, Clone)]
pub struct Ray<F: Float> {
    pub origin: Point<F>,
    pub direction: Vector<F>,
    pub inv_direction: Vector<F>,
}

impl<F: Float> Ray<F> {
    pub fn new(origin: Point<F>, dir: Vector<F>) -> Self {
        Self {
            origin,
            inv_direction: dir.recip(),
            direction: dir,
        }
    }

    pub fn point_at_distance(&self, t: F) -> Point<F> {
        self.origin + self.direction * t
    }
}
