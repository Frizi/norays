use fibers::sync::oneshot::MonitorError;
use fibers::Spawn;
use futures::{lazy, Future};
use light::{BounceQuota, Spectrum};
use math::{Float, Ray};
use scheduling::{Job, JobOut};
use shading::Material;
use std::marker::PhantomData;
use std::sync::Arc;
use tracing::{Hitable, Traceable};

pub struct TracingJob<F, H, M, T>
where
    F: Float,
    H: Hitable<F, Material = M>,
    M: Material<F>,
    T: Traceable<F, H, M> + Sync,
{
    ray: Ray<F>,
    traceable: Arc<T>,
    quota: BounceQuota,
    _h: PhantomData<H>,
    _m: PhantomData<M>,
}

impl<F, H, M, T> TracingJob<F, H, M, T>
where
    F: Float,
    H: Hitable<F, Material = M>,
    M: Material<F>,
    T: Traceable<F, H, M> + Sync,
{
    pub fn new(ray: Ray<F>, traceable: Arc<T>, quota: BounceQuota) -> Self {
        Self {
            ray,
            traceable,
            quota,
            _h: PhantomData,
            _m: PhantomData,
        }
    }
}

impl<F, H, M, T> Job for TracingJob<F, H, M, T>
where
    F: Float,
    H: Hitable<F, Material = M> + Send + 'static,
    M: Material<F> + Send + 'static,
    T: Traceable<F, H, M> + Sync + Send,
    Arc<T>: 'static,
{
    type Output = Option<Spectrum<F>>;

    fn schedule<HN: Spawn + Clone + Send + 'static>(self, handle: HN) -> JobOut<Self> {
        // type RetFut<F> = Box<Future<Item = Option<Spectrum<F>>, Error = ()> + Send>;
        let handle0 = handle.clone();
        let fiber = handle.spawn_monitor(lazy(move || {
            self.traceable.trace(&self.ray).map(|(dist, hitable)| {
                let hit = hitable.get_hit(&self.ray, dist);
                hit.evaluate_material(self.quota, handle0)
            })
        }));
        Box::new(fiber.map_err(|_: MonitorError<()>| ()))
    }
}
