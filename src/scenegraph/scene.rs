use light::BounceQuota;
use math::{Float, Point2D};
use scheduling::TracingJob;
use shading::Material;
use std::marker::PhantomData;
use std::sync::Arc;
use tracing::{Camera, Hitable, Traceable};

pub struct Scene<F, H, M, T, C>
where
    F: Float,
    H: Hitable<F, Material = M>,
    M: Material<F>,
    T: Traceable<F, H, M>,
    C: Camera<F>,
{
    camera: C,
    traceable: Arc<T>,
    quota: BounceQuota,
    _f: PhantomData<F>,
    _h: PhantomData<H>,
    _m: PhantomData<M>,
}

impl<F, H, M, T, C> Scene<F, H, M, T, C>
where
    F: Float,
    H: Hitable<F, Material = M>,
    M: Material<F>,
    T: Traceable<F, H, M>,
    C: Camera<F>,
{
    pub fn new(camera: C, traceable: T, quota: BounceQuota) -> Self {
        Self {
            camera,
            traceable: Arc::new(traceable),
            quota,
            _f: PhantomData,
            _h: PhantomData,
            _m: PhantomData,
        }
    }

    pub fn set_camera(&mut self, camera: C) {
        self.camera = camera
    }

    // pub fn render_into(&self, framebuffer: &mut Framebuffer) {
    //     framebuffer.fill(|p| {
    //         let ray = self.camera.screen_ray(&p);
    //         self.traceable
    //             .trace(&ray)
    //             .map_or(PURPLE, |hit| (&hit.normal).into())
    //     })
    // }
}

impl<F, H, M, T, C> Scene<F, H, M, T, C>
where
    F: Float,
    H: Hitable<F, Material = M>,
    M: Material<F>,
    T: Traceable<F, H, M> + Send + Sync,
    C: Camera<F> + Sync,
{
    pub fn job_for_fragment(&self, point: &Point2D<F>) -> TracingJob<F, H, M, T> {
        let ray = self.camera.screen_ray(&point);

        TracingJob::new(ray, self.traceable.clone(), self.quota.clone())
    }

    // pub fn prepare_render_into<'a, H: 'a + Spawn + Clone + Sync>(
    //     &'a self,
    //     framebuffer: &'a mut Framebuffer,
    //     handle: H,
    // ) -> Box<'a + Future<Item = Vec<u32>, Error = ()>> {
    //     let camera = Rc::new(self.camera.clone());

    //     framebuffer.par_fill(handle.clone(), move |p| {
    //         let ray = camera.screen_ray(&p);
    //         let job = TracingJob::new(
    //             ray,
    //             &self.traceable,
    //             TracingMode::Tristimulus,
    //             self.quota.clone(),
    //         );
    //         let monitor = handle.clone().spawn_monitor(job.run(handle.clone()));

    //         let f = monitor
    //             .map(|maybe_light| {
    //                 maybe_light.map_or(PURPLE, |light| {
    //                     let v = match light {
    //                         Light::Tristimulus(t) => Vector::new(t.x, t.y, t.z),
    //                         Light::Spectral(i) => {
    //                             Vector::new(i.intensity, i.intensity, i.intensity)
    //                         }
    //                     };

    //                     (&v).into()
    //                 })
    //             })
    //             .map_err(|_| ());

    //         Box::new(f)
    //     })
    // }
}
