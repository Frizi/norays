use drawing::Framebuffer;
use drawing::const_color::PURPLE;
use math::Float;
use std::marker::PhantomData;
use tracing::{Camera, Traceable};

pub struct Scene<F, T, C>
where
    F: Float,
    T: Traceable<F>,
    C: Camera<F>,
{
    camera: C,
    traceable: T,
    _float: PhantomData<F>,
}

unsafe impl<F, T, C> Sync for Scene<F, T, C>
where
    F: Float,
    T: Traceable<F>,
    C: Camera<F>,
{
}

impl<F, T, C> Scene<F, T, C>
where
    F: Float,
    T: Traceable<F>,
    C: Camera<F>,
{
    pub fn new(camera: C, traceable: T) -> Self {
        Self {
            camera,
            traceable,
            _float: PhantomData,
        }
    }

    pub fn set_camera(&mut self, camera: C) {
        self.camera = camera
    }

    pub fn render_into(&self, framebuffer: &mut Framebuffer) {
        framebuffer.par_fill(|p| {
            let ray = self.camera.screen_ray(&p);
            self.traceable
                .trace(&ray)
                .map_or(PURPLE, |hit| (&hit.normal).into())
        })
    }
}
