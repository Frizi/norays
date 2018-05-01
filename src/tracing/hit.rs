use fibers::Spawn;
use futures::Future;
use light::{BounceQuota, Spectrum};
use math::{Float, Point, Ray, Vector, UV};
use shading::Material;

#[derive(Clone, Copy)]
pub struct HitPoint<F: Float, M: Material<F>> {
    pub data: HitPointData<F>,
    pub material: M,
}

#[derive(Clone, Copy)]
pub struct HitPointData<F: Float> {
    pub point: Point<F>,
    pub normal: Vector<F>,
    pub incoming_dir: Vector<F>,
    pub uv: UV<F>,
}

impl<F: Float> HitPointData<F> {
    pub fn new(point: Point<F>, normal: Vector<F>, incoming_dir: Vector<F>, uv: UV<F>) -> Self {
        Self {
            point,
            normal,
            incoming_dir,
            uv,
        }
    }
}

impl<F: Float, M: Material<F>> HitPoint<F, M> {
    pub fn new(
        point: Point<F>,
        normal: Vector<F>,
        incoming_dir: Vector<F>,
        uv: UV<F>,
        material: M,
    ) -> Self {
        Self {
            data: HitPointData::new(point, normal, incoming_dir, uv),
            material,
        }
    }

    pub fn evaluate_material<H: Spawn + Clone + Send + 'static>(
        self,
        quota: BounceQuota,
        handle: H,
    ) -> Box<Future<Item = Spectrum<F>, Error = ()> + Send> {
        let material = self.material;
        material.evaluate(self.data, quota, handle)
    }
}
pub trait Hitable<F: Float>
where
    Self::Material: Material<F>,
{
    type Material;
    fn get_hit(&self, ray: &Ray<F>, distance: F) -> HitPoint<F, Self::Material>;
}
