extern crate minifb;
extern crate num_traits;
extern crate rayon;

pub mod drawing;
pub mod math;
pub mod scenegraph;
pub mod tracing;

use drawing::Framebuffer;
use math::{Aabb, Point, Vector};
use minifb::{Key, Window, WindowOptions};
use scenegraph::{Bvh, BvhNode, Scene, Sphere};
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

    let graph: Bvh<f32, Aabb<f32>, Sphere<f32>> = Bvh::from_nodes(vec![
        BvhNode::Leaf(Sphere::new(Point::new(-2.5, 0.0, 0.0), 1.0)),
        BvhNode::Leaf(Sphere::new(Point::new(0.0, 0.0, 0.0), 1.0)),
        BvhNode::Leaf(Sphere::new(Point::new(2.5, 0.0, 0.0), 1.0)),
    ]).unwrap();

    let mut scene = Scene::new(camera_for_time(aspect_ratio, start_time.elapsed()), graph);

    let seq_start = Instant::now();
    for _ in 0..100 {
        scene.render_into(&mut framebuffer);
    }
    let par_start = Instant::now();
    for _ in 0..100 {
        scene.render_into_par(&mut framebuffer);
    }
    let end = Instant::now();

    let seq_time = par_start - seq_start;
    let par_time = end - par_start;

    println!("====");
    println!("SEQ: {:?}", seq_time);
    println!("PAR: {:?}", par_time);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        scene.set_camera(camera_for_time(aspect_ratio, start_time.elapsed()));
        scene.render_into_par(&mut framebuffer);
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
    let dir = (lookat - &eye).normalized();

    PlaneCamera::new(eye, dir, Vector::plus_y(), aspect_ratio)
}
