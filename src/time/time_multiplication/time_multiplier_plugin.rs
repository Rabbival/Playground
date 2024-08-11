use strum::IntoEnumIterator;

use crate::prelude::*;

pub struct TimeMultiplierPlugin;

impl Plugin for TimeMultiplierPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, initialize_time_multipliers)
            .add_systems(
                Update,
                listen_for_time_multiplier_requests.in_set(TimerSystemSet::TimeMultipliersUpdating),
            );
    }
}

fn initialize_time_multipliers(mut commands: Commands) {
    for time_multiplier_id in TimeMultiplierId::iter() {
        commands.spawn(time_multiplier_id.to_initial_properties());
    }
}

fn listen_for_time_multiplier_requests(
    mut time_multiplier_set_request_reader: EventReader<SetTimeMultiplier>,
    mut event_from_timer_reader: EventReader<TimerGoingEvent<f32>>,
    mut time_multipliers: Query<(&mut TimeMultiplier, Entity)>,
    mut commands: Commands,
) {
    for time_multiplier_set_request in time_multiplier_set_request_reader.read() {
        fire_time_multiplier_changers(
            &time_multipliers,
            time_multiplier_set_request.id,
            time_multiplier_set_request.new_multiplier,
            time_multiplier_set_request.duration,
            &mut commands,
        );
    }
    for event_from_timer in event_from_timer_reader.read() {
        if let TimerGoingEventType::ChangeTimeMultiplierSpeed = event_from_timer.event_type {
            if let Ok([(mut multiplier, _)]) =
                time_multipliers.get_many_mut(event_from_timer.entities.get_initialized_subarray())
            {
                multiplier.set_value(event_from_timer.value);
            }
        }
    }
}

fn fire_time_multiplier_changers(
    time_multipliers: &Query<(&mut TimeMultiplier, Entity)>,
    id: TimeMultiplierId,
    new_multiplier: f32,
    duration: f32,
    commands: &mut Commands,
) {
    for (multiplier, multiplier_entity) in time_multipliers {
        if multiplier.id() == id {
            if multiplier.changeable() {
                commands.spawn(CalculatingTimer {
                    timer: FullTimer::new(
                        vec![multiplier_entity],
                        vec![TimeMultiplierId::default()],
                        duration,
                        TimerGoingEventType::ChangeTimeMultiplierSpeed,
                        TimerDoneEvent::default(),
                    ),
                    calculator: ValueByInterpolation::<f32>::new(
                        multiplier.value(),
                        new_multiplier,
                        Interpolator::default(),
                    ),
                });
            } else {
                print_warning(
                    TimeRelatedError::AttemptedToChangeFixedMultiplierTimeMultiplier(id),
                    vec![LogCategory::RequestNotFulfilled, LogCategory::Time],
                );
            }
            return;
        }
    }
    print_warning(
        TimeRelatedError::TimeMultiplierNotFound(id),
        vec![LogCategory::RequestNotFulfilled, LogCategory::Time],
    );
}
