use math::{Float, Ray};
use shading::Material;
use tracing::Hitable;

pub trait Traceable<F, H, M>
where
    F: Float,
    H: Hitable<F, Material = M>,
    M: Material<F>,
{
    fn trace(&self, ray: &Ray<F>) -> Option<(F, &H)>;
}
