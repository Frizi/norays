use num_traits::{Float as NTFloat, FloatConst};

pub trait Float: NTFloat + FloatConst + Send + Sync {}

impl<F: NTFloat + FloatConst + Send + Sync> Float for F {}
