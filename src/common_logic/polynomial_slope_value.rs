use crate::prelude::Numeric;

#[derive(Debug, Clone, Copy)]
pub struct PolynomialSlopeValue {
    pub power: f32,
}

impl PolynomialSlopeValue {
    pub fn new(power: f32) -> Self {
        Self { power }
    }

    pub fn calculate<T: Numeric>(&self, origin: T, goal: T, normalized_progress: f32) -> T {
        let delta = goal - origin;
        origin + delta * normalized_progress.powf(self.power)
    }
}

impl Default for PolynomialSlopeValue {
    fn default() -> Self {
        Self { power: 1.0 }
    }
}
