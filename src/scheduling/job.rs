use fibers::Spawn;
use futures::Future;

pub type JobOut<J> = Box<Future<Item = <J as Job>::Output, Error = ()> + Send>;

pub trait Job: Sized {
    type Output;
    fn schedule<H: Spawn + Clone + Send + 'static>(self, handle: H) -> JobOut<Self>;
}
