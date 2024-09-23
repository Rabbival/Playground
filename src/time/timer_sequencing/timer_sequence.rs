use crate::prelude::*;

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct TimerSequence {
    pub timers_in_order: VecBasedArray<EmittingTimer, MAX_TIMERS_IN_SEQUENCE>,
    pub current_timer_index: Option<usize>,
    pub loop_back_to_start: bool,
}

impl TimerSequence {
    pub fn new(timers_in_order_vec: Vec<EmittingTimer>, loop_back_to_start: bool) -> TimerSequence {
        let current_timer_index = if timers_in_order_vec.is_empty() {
            None
        } else {
            Some(1)
        };
        let timers_in_order_array = VecBasedArray::new(timers_in_order_vec);
        TimerSequence {
            timers_in_order: timers_in_order_array,
            current_timer_index,
            loop_back_to_start,
        }
    }
}
