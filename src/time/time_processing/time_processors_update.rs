use crate::{prelude::*, read_single_field_variant};

pub struct TimeMultipliersUpdatePlugin;

impl Plugin for TimeMultipliersUpdatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                listen_for_time_processors_requests::<f32>,
                listen_for_time_processors_requests::<Vec2>,
                listen_for_time_processors_requests::<Vec3>,
                listen_for_time_processors_requests::<Quat>,
            )
                .in_set(TimerSystemSet::TimeMultipliersUpdating),
        );
    }
}

fn listen_for_time_processors_requests<T: Numeric>(
    mut timer_event_reader: EventReader<TimerEventChannel<T>>,
    running_timers: Query<(&CustomTimer<T>, Entity)>,
    mut time_processors: ResMut<TimeMultipliers>,
    mut commands: Commands,
) {
    for time_processors_request in
        read_single_field_variant!(timer_event_reader, TimerEventChannel::ProcessorsRequest)
    {
        match time_processors_request {
            TimeMultipliersRequest::SetTimeMultiplier {
                processor_id,
                new_multiplier,
                duration,
            } => {
                destroy_ongoing_multiplier_changers(*processor_id, &running_timers, &mut commands);
                set_time_multiplier(
                    &mut time_processors,
                    &mut commands,
                    *processor_id,
                    *new_multiplier,
                    *duration,
                )
            }
            TimeMultipliersRequest::AddProcessor(time_processor) => {
                time_processors.add(*time_processor);
            }
        }
    }
}

fn set_time_multiplier(
    time_processors: &mut ResMut<TimeMultipliers>,
    commands: &mut Commands,
    processor_id: TimeMultiplierId,
    new_multiplier: f32,
    duration: f32,
) {
    let maybe_time_processor = time_processors.get_mut(processor_id);
    if let Some(time_processor) = maybe_time_processor {
        if time_processor.changeable() {
            commands.spawn(CustomTimer::<f32>::new(
                TimeMultiplierId::default(),
                duration,
                None,
                TimerValueCalculator::new(
                    time_processor.value(),
                    new_multiplier,
                    MathFunction::default(),
                ),
                Some(EventFromTimerType::ChangeTimeMultiplierSpeed(processor_id)),
                None,
            ));
        } else {
            print_warning(
                TimeRelatedError::AttemptedToChangeFixedMultiplierTimeMultiplier(processor_id),
                vec![LogCategory::RequestNotFulfilled],
            );
        }
    } else {
        print_warning(
            TimeRelatedError::TimeMultiplierNotFound(processor_id),
            vec![LogCategory::RequestNotFulfilled],
        );
    }
}

fn destroy_ongoing_multiplier_changers<T: Numeric>(
    processor_id: TimeMultiplierId,
    running_timers: &Query<(&CustomTimer<T>, Entity)>,
    commands: &mut Commands,
) {
    for (timer, timer_entity) in running_timers {
        if timer.send_as_going == Some(EventFromTimerType::ChangeTimeMultiplierSpeed(processor_id))
        {
            commands.entity(timer_entity).despawn();
        }
    }
}
