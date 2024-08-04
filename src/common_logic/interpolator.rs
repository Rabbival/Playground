use crate::prelude::Numeric;

#[derive(Debug, Clone, Copy)]
pub struct Interpolator {
    pub power: f32,
}

impl Interpolator {
    pub fn calculate<T: Numeric>(&self, origin: T, goal: T, normalized_progress: f32) -> T {
        let delta = goal - origin;
        origin + delta * normalized_progress.powf(self.power)
    }
}

impl Default for Interpolator {
    fn default() -> Self {
        Self { power: 1.0 }
    }
}
