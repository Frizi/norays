use fibers::Spawn;
use futures::{finished, Future};
use light::{BounceQuota, BounceType, Spectrum};
use math::Float;
use tracing::HitPointData;

pub trait Material<F: Float>: Sized + Clone {
    fn evaluate<H: Spawn + Clone>(
        &self,
        hit_point: HitPointData<F>,
        quota: BounceQuota,
        handle: H,
    ) -> Box<Future<Item = Spectrum<F>, Error = ()> + Send>;
}

#[derive(Clone)]
pub struct DebugNormalMaterial {}

impl<F: Float> Material<F> for DebugNormalMaterial {
    fn evaluate<H: Spawn + Clone>(
        &self,
        hit_point: HitPointData<F>,
        quota: BounceQuota,
        _handle: H,
    ) -> Box<Future<Item = Spectrum<F>, Error = ()> + Send> {
        let out = if let Some(_new_quota) = quota.attempt(BounceType::Diffuse) {
            // TODO: do something useful here
            let normal = hit_point.normal;
            let light = Spectrum {
                v: [normal.x, normal.y, normal.z, F::zero()],
            };
            light
        } else {
            Spectrum::zero()
        };

        Box::new(finished(out))
    }
}
