use color::ScreenSpaceColor;
use math::{Float, Point2D};

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

    pub fn write(&mut self, other: &Vec<u32>) {
        self.buffer.copy_from_slice(&other);
    }

    pub fn write_at<F: Float>(&mut self, point: &Point2D<F>, value: u32) {
        let x_opt = (point.x * F::from(self.width - 1).unwrap()).to_usize();
        let y_opt = (point.y * F::from(self.height - 1).unwrap()).to_usize();

        if let (Some(x), Some(y)) = (x_opt, y_opt) {
            let idx = x + y * self.width;
            self.buffer[idx] = value;
        }
    }

    pub fn fill<F, Func>(&mut self, mut func: Func)
    where
        F: Float,
        Func: FnMut(Point2D<F>) -> ScreenSpaceColor,
    {
        for (n, i) in self.buffer.iter_mut().enumerate() {
            let (x, y) = (n % self.width, n / self.width);
            let point = Point2D::new(
                // conversion from int to float will never fail
                F::from(x).unwrap() / F::from(self.width - 1).unwrap(),
                F::from(y).unwrap() / F::from(self.height - 1).unwrap(),
            );
            *i = func(point).as_rgb_u32();
        }
    }

    pub fn points<F: Float>(&self) -> impl Iterator<Item = Point2D<F>> {
        let (w, h) = (self.width, self.height);
        let points = (0..w * h).map(move |n| {
            let (x, y) = (n % w, n / w);
            Point2D::new(
                // conversion from int to float will never fail
                F::from(x).unwrap() / F::from(w).unwrap(),
                F::from(y).unwrap() / F::from(h).unwrap(),
            )
        });
        points
    }

    // pub fn par_fill<'a, H, F, Func>(
    //     &'a mut self,
    //     handle: H,
    //     func: Func,
    // ) -> Box<Future<Item = Vec<u32>, Error = ()> + 'a>
    // where
    //     H: Spawn + Clone + Sync + 'a,
    //     F: Float,
    //     Func: 'a + Fn(Point2D<F>) -> Box<Future<Item = Color, Error = ()> + Send>,
    // {
    //     let (w, h) = (self.width, self.height);

    //     let points = (0..w * h).map(|n| {
    //         let (x, y) = (n % w, n / w);
    //         Point2D::new(
    //             // conversion from int to float will never fail
    //             F::from(x).unwrap() / F::from(w).unwrap(),
    //             F::from(y).unwrap() / F::from(h).unwrap(),
    //         )
    //     });

    //     let jobs = points.map(|point| handle.spawn_monitor(func(point).map(|c| c.as_rgb_u32())));

    //     Box::new(join_all(jobs).map_err(|_| ()))

    // self.buffer
    //     .iter_mut()
    //     .enumerate()
    //     .scan(finished(()), |f, (n, i)| {
    // let (x, y) = (n % w, n / w);
    // let point = Point2D::new(
    //     // conversion from int to float will never fail
    //     F::from(x).unwrap() / F::from(w).unwrap(),
    //     F::from(y).unwrap() / F::from(h).unwrap(),
    // );

    //         f.join(handle.spawn_monitor(func(point)).map(|color| {
    //             *i = color.as_rgb_u32();
    //         }))
    //     });
    // }

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
