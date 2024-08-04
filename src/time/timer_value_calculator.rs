use crate::prelude::*;

#[derive(Default, Debug, Clone, Copy)]
pub struct TimerValueCalculator<T: Numeric> {
    original_value: T,
    goal_value: T,
    current_value_calculation: Interpolator,
}

impl<T: Numeric> TimerValueCalculator<T> {
    pub fn new(original_value: T, goal_value: T, current_value_calculation: Interpolator) -> Self {
        Self {
            original_value,
            goal_value,
            current_value_calculation,
        }
    }

    pub fn calculate_current_value(&self, normalized_progress: f32) -> T {
        self.current_value_calculation.calculate(
            self.original_value,
            self.goal_value,
            normalized_progress,
        )
    }
}
