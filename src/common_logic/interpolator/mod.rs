use crate::prelude::{clamp_and_notify, Numeric};

pub mod value_by_interpolation;

#[derive(Debug, Clone, Copy)]
pub struct Interpolator {
    pub power: f32,
}

impl Interpolator {
    pub fn new(power: f32) -> Self {
        Self { power }
    }

    pub fn calculate<T: Numeric>(&self, origin: T, goal: T, normalized_progress: f32) -> T {
        let clamped_normalized_progress = clamp_and_notify(normalized_progress, 0.0, 1.0);
        let delta = goal - origin;
        origin + delta * clamped_normalized_progress.powf(self.power)
    }
}

impl Default for Interpolator {
    fn default() -> Self {
        Self { power: 1.0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interpolator_for_out_of_bounds_progress() {
        let interpolator = Interpolator::default();
        let start_value = 1.0;
        let end_value = 42.0;
        let endpoint = interpolator.calculate(start_value, end_value, 3.14);
        assert_eq!(endpoint, end_value);
        let startpoint = interpolator.calculate(start_value, end_value, -3.14);
        assert_eq!(startpoint, start_value);
    }

    #[test]
    fn test_linear_interpolator() {
        let linear_interpolator = Interpolator::new(1.0);
        let midpoint = linear_interpolator.calculate(0.0, 1.0, 0.5);
        assert_eq!(midpoint, 0.5);
    }

    #[test]
    fn test_parabolic_interpolator() {
        let parabolic_interpolator = Interpolator::new(2.0);
        let midpoint = parabolic_interpolator.calculate(1.0, 2.0, 0.5);
        assert_eq!(midpoint, 1.25);
    }

    #[test]
    fn test_radical_interpolator() {
        let radical_interpolator = Interpolator::new(0.5);
        let quarter_from_start_point = radical_interpolator.calculate(1.0, 2.0, 0.25);
        assert_eq!(quarter_from_start_point, 1.5);
    }
}
