use light::spds::{SPDBase, SPD};

pub struct RegularSPD {
    inner: SPDBase,
    sample_cache: Vec<f32>,
}

impl SPD for RegularSPD {
    fn base(&self) -> &SPDBase {
        &self.inner
    }
    fn samples(&self) -> &[f32] {
        &self.sample_cache
    }
    fn samples_mut(&mut self) -> &mut [f32] {
        &mut self.sample_cache
    }
}

impl RegularSPD {
    pub fn new(s: &[f32], lambda_min: f32, lambda_max: f32, scale: f32) -> Self {
        let num_samples = s.len();
        let delta = (lambda_max - lambda_min) / (num_samples as f32 - 1.0);

        let mut out = Self {
            inner: SPDBase::new(lambda_min, lambda_max, delta),
            sample_cache: Vec::from(s),
        };
        out.scale(scale);
        out
    }
}
