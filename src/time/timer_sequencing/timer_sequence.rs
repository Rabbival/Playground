use crate::prelude::*;

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct TimerSequence {
    pub timers_in_order: VecBasedArray<EmittingTimer, MAX_TIMERS_IN_SEQUENCE>,
    pub loop_back_to_start: bool,
}

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct SequenceIsDone(bool);

impl TimerSequence {
    pub fn new(timers_in_order_vec: Vec<EmittingTimer>, loop_back_to_start: bool) -> TimerSequence {
        let timers_in_order_array = VecBasedArray::new(timers_in_order_vec);
        TimerSequence {
            timers_in_order: timers_in_order_array,
            loop_back_to_start,
        }
    }

    pub fn fire_first(
        &self,
        timer_fire_event_writer: &mut EventWriter<TimerFireRequest>,
    ) -> Result<(), TimerSequenceError> {
        self.fire_by_index(0, timer_fire_event_writer)
    }

    pub fn fire_next_timer_in_sequence(
        &self,
        timer_fire_event_writer: &mut EventWriter<TimerFireRequest>,
        done_timer_index: usize,
    ) -> Result<SequenceIsDone, TimerSequenceError> {
        let next_index = done_timer_index + 1;
        let sequence_timer_count = self.timers_in_order.len();
        if next_index > sequence_timer_count {
            if self.loop_back_to_start {
                self.fire_first(timer_fire_event_writer)?;
                Ok(SequenceIsDone(false))
            } else {
                print_info("Timer sequence done", vec![LogCategory::Time]);
                Ok(SequenceIsDone(true))
            }
        } else {
            self.fire_by_index(next_index, timer_fire_event_writer)?;
            Ok(SequenceIsDone(false))
        }
    }

    fn fire_by_index(
        &self,
        index: usize,
        timer_fire_event_writer: &mut EventWriter<TimerFireRequest>,
    ) -> Result<(), TimerSequenceError> {
        let timers_array = self.timers_in_order.array;
        match timers_array[index] {
            Some(timer) => {
                timer_fire_event_writer.send(TimerFireRequest(timer));
                Ok(())
            }
            None => Err(TimerSequenceError::SequenceHasNoTimerInIndex(index)),
        }
    }
}
