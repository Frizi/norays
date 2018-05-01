use color::XYZColor;
use light::spds::data::{CIE_COUNT, CIE_START, CIE_X, CIE_Y, CIE_Z};
use light::spds::SPDBase;
use light::spectrum_wavelengths::SCALE_W_TO_LM;
use math::Float;
use std::cmp::Ordering::Equal;

pub trait SPD {
	fn samples(&self) -> &[f32];
	fn samples_mut(&mut self) -> &mut [f32];
	fn base(&self) -> &SPDBase;

	fn lambda_min(&self) -> f32 {
		self.base().lambda_min()
	}
	fn lambda_max(&self) -> f32 {
		self.base().lambda_max()
	}
	fn delta(&self) -> f32 {
		self.base().delta()
	}
	fn inv_delta(&self) -> f32 {
		self.base().inv_delta()
	}

	fn normalize(&mut self) {
		if let Some(max) = self.samples()
			.iter()
			.max_by(|a, b| a.partial_cmp(b).unwrap_or(Equal))
			.map(|&x| x)
		{
			self.scale(1.0 / max);
		}
	}

	fn clamp(&mut self) {
		self.samples_mut().iter_mut().for_each(|s| {
			if !(*s > 0.0) {
				*s = 0.0
			}
		});
	}

	fn scale(&mut self, scale: f32) {
		self.samples_mut().iter_mut().for_each(|s| *s *= scale);
	}

	// luxcore's Y
	fn luma(&self) -> f32 {
		(0..CIE_COUNT)
			.map(|i| self.sample((i + CIE_START) as _) * CIE_Y[i])
			.sum::<f32>() * SCALE_W_TO_LM
	}

	fn filter(&self) -> f32 {
		let samples = self.samples();
		let sum: f32 = samples.iter().sum();
		sum / samples.len() as f32
	}
	fn sample(&self, lambda: f32) -> f32 {
		if lambda < self.lambda_min() || lambda > self.lambda_max() {
			return 0.0;
		}
		let samples = self.samples();
		let x = (lambda - self.lambda_min()) * self.inv_delta();
		let b0 = x as usize;
		let b1 = (b0 + 1).min(samples.len() - 1);
		let dx = x - b0 as f32;
		return dx.lerp(samples[b0], samples[b1]);
	}

	fn to_xyz(&self) -> XYZColor {
		(0..CIE_COUNT)
			.map(|i| {
				let s = self.sample((i + CIE_START) as _);
				XYZColor::new(s * CIE_X[i], s * CIE_Y[i], s * CIE_Z[i])
			})
			.sum::<XYZColor>() * SCALE_W_TO_LM
	}
	fn to_normalized_xyz(&self) -> XYZColor {
		(0..CIE_COUNT)
			.map(|i| {
				let s = self.sample((i + CIE_START) as _);
				XYZColor::new(s * CIE_X[i], s * CIE_Y[i], s * CIE_Z[i])
			})
			.sum::<XYZColor>() / CIE_Y.iter().sum()
	}
}
