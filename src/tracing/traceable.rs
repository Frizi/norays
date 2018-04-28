use math::{Float, Ray};
use tracing::Hit;

pub trait Traceable<F: Float> {
    fn trace(&self, ray: &Ray<F>) -> Option<Hit<F>>;
}
