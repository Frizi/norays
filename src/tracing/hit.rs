use math::{Float, Vector};

pub struct Hit<F: Float> {
    pub distance: F,
    pub normal: Vector<F>,
}
