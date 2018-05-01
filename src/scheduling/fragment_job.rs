// use fibers::Spawn;
// use futures::{Future, finished, Future};
// use math::{Float, Point2D};
// use scheduling::TracingJob;
// use tracing::Traceable;

// pub struct FragmentJob<'a, F, T>
// where
//     F: Float,
//     T: 'a + Traceable<F>,
// {
//     point: Point2D<F>,
//     tracing_job: TracingJob<'a, F, T>,
// }

// impl<'a, F, T> FragmentJob<'a, F, T>
// where
//     F: Float,
//     T: 'a + Traceable<F>,
// {
//     pub fn new(point: Point2D<F>, tracing_job: TracingJob<'a, F, T>) -> Self {
//         Self { point, tracing_job }
//     }

//     pub fn run<H: Spawn + Clone>(&self, handle: H, mspc::send) -> Box<Future<Item = (), Error = ()>> {

//     }
// }
