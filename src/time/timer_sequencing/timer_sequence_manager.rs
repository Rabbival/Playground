use crate::prelude::*;

pub struct TimerSequenceManagerPlugin;

impl Plugin for TimerSequenceManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            listen_for_done_sequence_timers.in_set(EndOfFrameSystemSet::TimerClearing),
        );
    }
}

pub fn listen_for_done_sequence_timers(
    mut event_reader: EventReader<TimerDoneEvent>,
    mut timer_fire_event_writer: EventWriter<TimerFireRequest>,
    mut destroy_calculator_event_writer: EventWriter<DestroyValueCalculator>,
    timer_sequence_query: Query<(&TimerSequence, Entity)>,
    mut commands: Commands,
) {
    for timer_parent_sequence in event_reader
        .read()
        .filter_map(|done_event| done_event.timer_parent_sequence)
    {
        if let Ok((timer_sequence, sequence_entity)) =
            timer_sequence_query.get(timer_parent_sequence.parent_sequence)
        {
            if let Err(timer_sequence_error) = advance_sequence(
                &mut timer_fire_event_writer,
                &mut destroy_calculator_event_writer,
                timer_parent_sequence.index_in_sequence,
                sequence_entity,
                timer_sequence,
                &mut commands,
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

fn advance_sequence(
    timer_fire_event_writer: &mut EventWriter<TimerFireRequest>,
    destroy_calculator_event_writer: &mut EventWriter<DestroyValueCalculator>,
    done_timer_index: usize,
    sequence_entity: Entity,
    timer_sequence: &TimerSequence,
    commands: &mut Commands,
) -> Result<(), TimerSequenceError> {
    let sequence_status = timer_sequence.get_next_timer_index(done_timer_index);
    if let Some(next_index) = sequence_status.next_timer_index {
        fire_next_timer(
            timer_fire_event_writer,
            next_index,
            sequence_entity,
            timer_sequence,
        )?;
    }
    if sequence_status.sequence_done {
        for timer in timer_sequence.timers_in_order.iter() {
            for value_calculator_entity in timer.calculator_entities_iter() {
                destroy_calculator_event_writer
                    .send(DestroyValueCalculator(value_calculator_entity));
            }
        }
        despawn_entity_notify_on_fail(sequence_entity, "timer sequence", commands);
    }
    Ok(())
}

fn fire_next_timer(
    timer_fire_event_writer: &mut EventWriter<TimerFireRequest>,
    next_index: usize,
    sequence_entity: Entity,
    timer_sequence: &TimerSequence,
) -> Result<(), TimerSequenceError> {
    let timer = timer_sequence.get_timer_by_index(next_index)?;
    print_info(
        format!("Firing timer: {:?}", timer),
        vec![LogCategory::Time],
    );

    //DEBUG
    println!(
        "previous timer done, requesting firing of timer with index {:?}",
        next_index
    );

    timer_fire_event_writer.send(TimerFireRequest {
        timer,
        parent_sequence: Some(TimerParentSequence {
            parent_sequence: sequence_entity,
            index_in_sequence: next_index,
        }),
    });
    Ok(())
}
