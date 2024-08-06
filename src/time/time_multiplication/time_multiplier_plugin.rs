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
    mut event_from_timer_reader: EventReader<EventFromTimer<f32>>,
    mut add_timer_event_writer: EventWriter<AddTimerToEntity<f32>>,
    mut time_multipliers: Query<(&mut TimeMultiplier, Entity)>,
) {
    for time_multiplier_set_request in time_multiplier_set_request_reader.read() {
        fire_time_multiplier_changers(
            &mut add_timer_event_writer,
            &time_multipliers,
            time_multiplier_set_request.id,
            time_multiplier_set_request.new_multiplier,
            time_multiplier_set_request.duration,
        );
    }
    for event_from_timer in event_from_timer_reader.read() {
        if let Some(EventFromTimerType::ChangeTimeMultiplierSpeed) =
            event_from_timer.try_get_as_going_event()
        {
            if let Ok((mut multiplier, _)) = time_multipliers.get_mut(event_from_timer.entity()) {
                multiplier.set_value(event_from_timer.current_value());
            }
        }
    }
}

fn fire_time_multiplier_changers(
    add_timer_event_writer: &mut EventWriter<AddTimerToEntity<f32>>,
    time_multipliers: &Query<(&mut TimeMultiplier, Entity)>,
    id: TimeMultiplierId,
    new_multiplier: f32,
    duration: f32,
) {
    for (multiplier, multiplier_entity) in time_multipliers {
        if multiplier.id() == id {
            if multiplier.changeable() {
                add_timer_event_writer.send(AddTimerToEntity {
                    timer: CustomTimer::<f32>::new(
                        TimeMultiplierId::default(),
                        duration,
                        TimerValueCalculator::new(
                            multiplier.value(),
                            new_multiplier,
                            Interpolator::default(),
                        ),
                        Some(EventFromTimerType::ChangeTimeMultiplierSpeed),
                        None,
                    ),
                    entity: multiplier_entity,
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
