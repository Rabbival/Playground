use crate::{plugin_for_implementors_of_trait, prelude::*};

plugin_for_implementors_of_trait!(GoingEventValueCalculatorsPlugin, Numeric);

impl<T: Numeric> Plugin for GoingEventValueCalculatorsPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            clear_done_calculators::<T>.in_set(EndOfFrameSystemSet::PostTimerClearing),
        );
    }
}

fn clear_done_calculators<T: Numeric>(
    mut event_reader: EventReader<DestroyValueCalculator>,
    timer_value_calculators: Query<&GoingEventValueCalculator<T>>,
    mut commands: Commands,
) {
    for calculators_clearing_request in event_reader.read() {
        if timer_value_calculators.contains(calculators_clearing_request.0) {
            despawn_entity_notify_on_fail(
                calculators_clearing_request.0,
                "an EmittingTimer's ValueCalculator",
                &mut commands,
            );
        }
    }
}
