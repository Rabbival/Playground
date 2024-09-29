use crate::prelude::*;

pub struct TimerSequenceManagerPlugin;

impl Plugin for TimerSequenceManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, listen_for_done_sequence_timers);
    }
}

fn listen_for_done_sequence_timers(
    mut event_reader: EventReader<TimerDoneEvent>,
    mut timer_fire_event_writer: EventWriter<TimerFireRequest>,
    timer_sequence_query: Query<&TimerSequence>,
) {
    for timer_parent_sequence in event_reader
        .read()
        .filter_map(|done_event| done_event.timer_parent_sequence)
    {
        if let Ok(timer_sequence) = timer_sequence_query.get(timer_parent_sequence.parent_sequence)
        {
            if let Err(timer_sequence_error) = timer_sequence.fire_next_timer_in_sequence(
                &mut timer_fire_event_writer,
                timer_parent_sequence.index_in_sequence,
            ) {
                print_error(
                    timer_sequence_error,
                    vec![LogCategory::Time, LogCategory::RequestNotFulfilled],
                )
            }
        } else {
            print_error(
                EntityError::EntityNotInQuery("timer sequence of a done timer"),
                vec![LogCategory::RequestNotFulfilled],
            );
        }
    }
}
