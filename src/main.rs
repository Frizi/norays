#[macro_use]
extern crate lazy_static;
extern crate fibers;
extern crate futures;
extern crate minifb;
extern crate nbchan;
extern crate num_traits;

pub mod color;
pub mod drawing;
pub mod light;
pub mod math;
pub mod scenegraph;
pub mod scheduling;
pub mod shading;
pub mod tracing;

use color::ScreenSpaceColor;
use drawing::Framebuffer;
use fibers::{Executor, Spawn, ThreadPoolExecutor};
use futures::Future;
use light::BounceQuota;
use math::{Aabb, Point, Vector};
use minifb::{Key, Window, WindowOptions};
use nbchan::mpsc as nb_mpsc;
use scenegraph::{Bvh, BvhNode, Scene, ShadedSphere};
use scheduling::Job;
use shading::DebugNormalMaterial;
use std::time::{Duration, Instant};
use tracing::PlaneCamera;

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

fn main() {
    let start_time = Instant::now();

    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT);
    let aspect_ratio = WIDTH as f32 / HEIGHT as f32;

    let mut window =
        Window::new("NoRays", WIDTH, HEIGHT, WindowOptions::default()).unwrap_or_else(|e| {
            panic!("{}", e);
        });

    let mat = DebugNormalMaterial {};

    let graph: Bvh<f32, Aabb<f32>, _, _, _> = Bvh::from_nodes(vec![
        BvhNode::Leaf(ShadedSphere::new(
            Point::new(-2.5, 0.0, 0.0),
            1.0,
            mat.clone(),
        )),
        BvhNode::Leaf(ShadedSphere::new(
            Point::new(0.0, 0.0, 0.0),
            1.0,
            mat.clone(),
        )),
        BvhNode::Leaf(ShadedSphere::new(
            Point::new(2.5, 0.0, 0.0),
            1.0,
            mat.clone(),
        )),
    ]).unwrap();

    let quota = BounceQuota::new(30, 5, 5, 5);

    let scene = Scene::new(
        camera_for_time(aspect_ratio, start_time.elapsed()),
        graph,
        quota,
    );

    let mut executor = ThreadPoolExecutor::new().expect("Cannot create Executor");
    // let mut sync_executor = InPlaceExecutor::new().expect("Cannot create Sync Executor");

    let handle = executor.handle();
    // let sync_handle = sync_executor.handle();

    let (pixel_tx, pixel_rx) = nb_mpsc::channel();

    framebuffer.points().for_each(|point| {
        let job = scene.job_for_fragment(&point);

        let tx = pixel_tx.clone();
        let pixel_future = job.schedule(handle.clone())
            .and_then(|maybe_light| {
                if let Some(spectrum) = maybe_light {
                    let vec = Vector::new(spectrum.v[0], spectrum.v[1], spectrum.v[2]);
                    let color: ScreenSpaceColor = vec.into();

                    Ok((point, color.as_rgb_u32()))
                } else {
                    Err(())
                }
            })
            .and_then(move |tuple| tx.send(tuple).map_err(|_| ()));

        handle.spawn_monitor(pixel_future);
    });

    // let mut render_promise = scene.prepare_render_into(&mut framebuffer, executor.handle());

    let max_dur = Duration::from_millis(16);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let start_time = Instant::now();

        while start_time.elapsed() < max_dur {
            executor.run_once().expect("Error while execution");
        }

        while let Ok((point, value)) = pixel_rx.try_recv() {
            framebuffer.write_at(&point, value)
        }
        // if let Ok(Async::Ready(x)) = render_promise.poll() {
        //     framebuffer.write(&x);
        // }
        // scene.set_camera(camera_for_time(aspect_ratio, start_time.elapsed()));
        window.update_with_buffer(framebuffer.raw_buffer()).unwrap();
    }
}

fn camera_for_time(aspect_ratio: f32, time: Duration) -> PlaneCamera<f32> {
    let ms = (time.as_secs() * 1_000_000_000 + time.subsec_nanos() as u64) / 1000;

    let wobble = (ms as f32 * 3e-7).sin() * 2.5;
    let angle = ms as f32 * 7e-7;

    let r = 10.0;

    let (x, z) = (angle.cos() * r, angle.sin() * r);

    let eye = Point::new(x, 5.0, z);
    let lookat = Point::new(wobble, 0.0, 0.0);
    let dir = (lookat - eye).normalized();

    PlaneCamera::new(eye, dir, Vector::plus_y(), aspect_ratio)
}
