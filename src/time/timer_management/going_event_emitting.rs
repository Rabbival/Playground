use crate::{plugin_for_implementors_of_trait, prelude::*};

plugin_for_implementors_of_trait!(TimerTickingAndClearingGenericPlugin, Numeric);

impl<T: Numeric> Plugin for TimerTickingAndClearingGenericPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            calculate_value_and_send_going_event::<T>.in_set(TimerSystemSet::PostTickingImmidiate),
        );
    }
}

pub fn calculate_value_and_send_going_event<T: Numeric>(
    mut calculation_requests_reader: EventReader<CalculateAndSendGoingEvent>,
    mut timer_going_event_writer: EventWriter<TimerGoingEvent<T>>,
    value_calculators: Query<&ValueByInterpolation<T>>,
) {
    for calculation_request in calculation_requests_reader.read() {
        for affected_entity in calculation_request.affected_entities.iter() {
            if let Ok(value_calculator) =
                value_calculators.get(affected_entity.value_calculator_entity)
            {
                let calculated_value = value_calculator
                    .calculate_current_value(calculation_request.normalized_progress);
                timer_going_event_writer.send(TimerGoingEvent::<T> {
                    event_type: calculation_request.event_type_to_send,
                    entity: affected_entity.affected_entity,
                    value: calculated_value,
                });
            }
        }
    }
}
