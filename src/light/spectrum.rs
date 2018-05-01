use math::Float;

const WAVELENGTH_SAMPLES: usize = 4;

#[derive(Debug)]
pub struct Spectrum<F: Float> {
    pub v: [F; WAVELENGTH_SAMPLES],
}

impl<F: Float> Spectrum<F> {
    pub fn zero() -> Self {
        Self {
            v: [F::zero(); WAVELENGTH_SAMPLES],
        }
    }
}
