use drawing::Color;
use math::{Float, Point2D};
use rayon::prelude::*;

pub struct Framebuffer {
    buffer: Vec<u32>,
    width: usize,
    height: usize,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Framebuffer {
        Framebuffer {
            buffer: vec![0; width * height],
            width,
            height,
        }
    }

    pub fn fill<F, Func>(&mut self, mut func: Func)
    where
        F: Float,
        Func: FnMut(Point2D<F>) -> Color,
    {
        for (n, i) in self.buffer.iter_mut().enumerate() {
            let (x, y) = (n % self.width, n / self.width);
            let point = Point2D::new(
                // conversion from int to float will never fail
                F::from(x).unwrap() / F::from(self.width).unwrap(),
                F::from(y).unwrap() / F::from(self.height).unwrap(),
            );
            *i = func(point).as_rgb_u32();
        }
    }

    pub fn par_fill<F, Func>(&mut self, func: Func)
    where
        F: Float,
        Func: Fn(Point2D<F>) -> Color + Sync,
    {
        let (w, h) = (self.width, self.height);
        self.buffer.par_iter_mut().enumerate().for_each(|(n, i)| {
            let (x, y) = (n % w, n / w);
            let point = Point2D::new(
                // conversion from int to float will never fail
                F::from(x).unwrap() / F::from(w).unwrap(),
                F::from(y).unwrap() / F::from(h).unwrap(),
            );
            *i = func(point).as_rgb_u32();
        })
    }

    pub fn raw_buffer(&self) -> &Vec<u32> {
        &self.buffer
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}
