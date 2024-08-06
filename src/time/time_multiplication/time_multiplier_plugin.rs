use strum::IntoEnumIterator;

use crate::{prelude::*, read_struct_variant, read_two_field_variant};

pub struct TimeMultiplierPlugin;

impl Plugin for TimeMultiplierPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, initialize_time_multipliers)
            // .add_systems(
            //     Update,
            //     listen_for_time_multiplier_requests.in_set(TimerSystemSet::TimeMultipliersUpdating),
            // );
            ;
    }
}

fn initialize_time_multipliers(mut commands: Commands) {
    for time_multiplier_id in TimeMultiplierId::iter() {
        commands.spawn(time_multiplier_id.to_initial_properties());
    }
}

fn listen_for_time_multiplier_requests(
    mut time_event_reader: EventReader<TimeEventChannel<f32>>,
    mut time_event_writer: EventWriter<TimeEventChannel<f32>>,
    mut time_multipliers: Query<(&mut TimeMultiplier, Entity)>,
) {
    for (id, new_multiplier, duration) in read_struct_variant!(
        time_event_reader,
        TimeEventChannel::SetTimeMultiplier,
        id,
        new_multiplier,
        duration
    ) {
        fire_time_multiplier_changers(
            &mut time_event_writer,
            &time_multipliers,
            *id,
            *new_multiplier,
            *duration,
        );
    }
    for (&entity, event_from_timer) in
        read_two_field_variant!(time_event_reader, TimeEventChannel::EventFromTimer)
    {
        if let Some(EventFromTimerType::ChangeTimeMultiplierSpeed) =
            event_from_timer.try_get_as_going_event()
        {
            if let Ok((mut multiplier, _)) = time_multipliers.get_mut(entity) {
                multiplier.set_value(event_from_timer.current_value());
            }
        }
    }
}

fn fire_time_multiplier_changers(
    time_event_writer: &mut EventWriter<TimeEventChannel<f32>>,
    time_multipliers: &Query<(&mut TimeMultiplier, Entity)>,
    id: TimeMultiplierId,
    new_multiplier: f32,
    duration: f32,
) {
    for (multiplier, multiplier_entity) in time_multipliers {
        if multiplier.id() == id {
            if multiplier.changeable() {
                time_event_writer.send(TimeEventChannel::AddTimerToEntity(
                    multiplier_entity,
                    CustomTimer::<f32>::new(
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
                ));
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
