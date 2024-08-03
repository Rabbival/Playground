use crate::prelude::Numeric;

#[derive(Debug, Default, Clone, Copy)]
pub enum MathFunction {
    #[default]
    Linear,
    Parabolic {
        power: f32,
    },
}

impl MathFunction {
    pub fn calculate<T: Numeric>(&self, origin: T, goal: T, normalized_progress: f32) -> T {
        let delta = goal - origin;
        match self {
            Self::Linear => origin + delta * normalized_progress,
            Self::Parabolic { power } => {
                let parabolic_delta = normalized_progress.powf(*power);
                origin + delta * parabolic_delta
            }
        }
    }
}
