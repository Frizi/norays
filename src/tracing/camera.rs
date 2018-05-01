use math::{Float, Point, Point2D, Ray, Vector};

pub trait Camera<F: Float>: Clone {
    fn screen_ray(&self, screen_pos: &Point2D<F>) -> Ray<F>;
}

#[derive(Debug, Clone)]
pub struct PlaneCamera<F: Float> {
    origin: Point<F>,
    plane_origin: Point<F>,
    plane_x: Vector<F>,
    plane_y: Vector<F>,
}

impl<F: Float> PlaneCamera<F> {
    pub fn new(origin: Point<F>, direction: Vector<F>, up: Vector<F>, aspect_ratio: F) -> Self {
        let half = (F::one() + F::one()).recip();
        let plane_x = up.cross(direction).normalized();
        let plane_y = plane_x.cross(direction).normalized() * aspect_ratio.recip();
        let plane_origin = origin + (direction - (plane_x + plane_y) * half);
        Self {
            origin,
            plane_x,
            plane_y,
            plane_origin,
        }
    }
}

impl<F: Float> Camera<F> for PlaneCamera<F> {
    fn screen_ray(&self, screen_pos: &Point2D<F>) -> Ray<F> {
        let plane_point =
            self.plane_origin + (self.plane_x * screen_pos.x) + (self.plane_y * screen_pos.y);
        let dir = (plane_point - self.origin).normalized();
        Ray::new(self.origin.clone(), dir)
    }
}
