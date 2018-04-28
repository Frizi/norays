use num_traits::{Float as NTFloat, FloatConst};

pub trait Float: NTFloat + FloatConst {}

impl<F: NTFloat + FloatConst> Float for F {}
