pub struct SPDBase {
    lambda_min: f32,
    lambda_max: f32,
    delta: f32,
    inv_delta: f32,
}

impl SPDBase {
    pub fn new(lambda_min: f32, lambda_max: f32, delta: f32) -> Self {
        Self {
            lambda_min,
            lambda_max,
            delta,
            inv_delta: 1.0 / delta,
        }
    }

    pub fn lambda_min(&self) -> f32 {
        self.lambda_min
    }
    pub fn lambda_max(&self) -> f32 {
        self.lambda_max
    }
    pub fn delta(&self) -> f32 {
        self.delta
    }
    pub fn inv_delta(&self) -> f32 {
        self.inv_delta
    }
}
