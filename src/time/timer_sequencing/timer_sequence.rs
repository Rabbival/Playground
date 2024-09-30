use crate::prelude::*;

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct TimerSequence {
    pub timers_in_order: VecBasedArray<EmittingTimer, MAX_TIMERS_IN_SEQUENCE>,
    pub loop_back_to_start: bool,
}

impl TimerSequence {
    pub fn spawn_sequence_and_fire_first_timer(
        timer_fire_event_writer: &mut EventWriter<TimerFireRequest>,
        timers_in_order_vec: &[EmittingTimer],
        loop_back_to_start: bool,
        commands: &mut Commands,
    ) -> Result<(), TimerSequenceError> {
        let newborn_sequence = Self::new(timers_in_order_vec, loop_back_to_start);
        let newborn_sequence_id = commands.spawn(newborn_sequence).id();
        match timers_in_order_vec.first() {
            Some(timer) => {
                timer_fire_event_writer.send(TimerFireRequest {
                    timer: *timer,
                    parent_sequence: Some(TimerParentSequence {
                        parent_sequence: newborn_sequence_id,
                        index_in_sequence: 0,
                    }),
                });
                Ok(())
            }
            None => Err(TimerSequenceError::TriedToFireATimerSequenceWithNoTimers),
        }
    }

    fn new(timers_in_order_vec: &[EmittingTimer], loop_back_to_start: bool) -> TimerSequence {
        let timers_in_order_array = VecBasedArray::new(timers_in_order_vec.to_vec());
        TimerSequence {
            timers_in_order: timers_in_order_array,
            loop_back_to_start,
        }
    }

    pub fn get_timer_by_index(&self, index: usize) -> Result<EmittingTimer, TimerSequenceError> {
        match self.timers_in_order.array[index] {
            Some(timer) => Ok(timer),
            None => Err(TimerSequenceError::SequenceHasNoTimerInIndex(index)),
        }
    }

    pub fn get_next_timer_index(&self, done_timer_index: usize) -> TimerSequenceStatus {
        let next_index = done_timer_index + 1;
        let sequence_timer_count = self.timers_in_order.len();
        if next_index >= sequence_timer_count {
            if self.loop_back_to_start {
                TimerSequenceStatus {
                    next_timer_index: Some(0),
                    sequence_done: false,
                }
            } else {
                print_info("Timer sequence done", vec![LogCategory::Time]);
                TimerSequenceStatus {
                    next_timer_index: None,
                    sequence_done: true,
                }
            }
        } else {
            TimerSequenceStatus {
                next_timer_index: Some(next_index),
                sequence_done: false,
            }
        }
    }
}
