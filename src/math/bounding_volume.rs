use math::{Float, Ray};

pub trait BoundingVolume<F: Float>: Clone + Copy {
    fn combine(&self, rhs: &Self) -> Self;
    fn estimated_volume(&self) -> F;
    fn test(&self, against: &Ray<F>) -> bool;
}

pub trait Bounded<F: Float, T: BoundingVolume<F>> {
    fn bounding_volume(&self) -> T;
}

pub trait BoundingVolumeSum<F: Float, B: BoundingVolume<F>> {
    fn bounding_sum(self) -> Option<B>;
}

impl<'a, F: Float, B: BoundingVolume<F>, I: Iterator<Item = B>> BoundingVolumeSum<F, B> for I {
    fn bounding_sum(self) -> Option<B> {
        self.fold(None, |last, bound| match last {
            None => Some(bound),
            Some(b) => Some(b.combine(&bound)),
        })
    }
}
