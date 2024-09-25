use crate::prelude::*;

pub struct TimerSequenceManagerPlugin;

impl Plugin for TimerSequenceManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, listen_for_done_sequence_timers);
    }
}

fn listen_for_done_sequence_timers(
    mut event_reader: EventReader<TimerDoneEvent>,
    mut event_writer: EventWriter<TimerFireRequest>,
    timer_sequence_query: Query<&TimerSequence>,
) {
    for timer_parent_sequence in event_reader
        .read()
        .filter_map(|done_event| done_event.timer_parent_sequence)
    {
        if let Ok(timer_sequence) = timer_sequence_query.get(timer_parent_sequence.parent_sequence)
        {
            if let Some(timer_to_fire) =
                determine_timer_to_fire(timer_parent_sequence.index_in_sequence + 1, timer_sequence)
            {
                event_writer.send(TimerFireRequest(timer_to_fire));
            }
        } else {
            print_error(
                EntityError::EntityNotInQuery("timer sequence of a done timer"),
                vec![LogCategory::RequestNotFulfilled],
            );
        }
    }
}

fn determine_timer_to_fire(
    next_index: usize,
    timer_sequence: &TimerSequence,
) -> Option<EmittingTimer> {
    let sequence_timer_count = timer_sequence.timers_in_order.len();
    let timers_array = timer_sequence.timers_in_order.array;
    if next_index == sequence_timer_count {
        if timer_sequence.loop_back_to_start {
            Some(timers_array[0].unwrap())
        } else {
            None
        }
    } else if next_index <= sequence_timer_count {
        Some(timers_array[next_index].unwrap())
    } else {
        None
    }
}
