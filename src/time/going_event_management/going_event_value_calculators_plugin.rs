use crate::{plugin_for_implementors_of_trait, prelude::*, read_single_field_variant};

plugin_for_implementors_of_trait!(GoingEventValueCalculatorsPlugin, Numeric);

impl<T: Numeric> Plugin for GoingEventValueCalculatorsPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                initialize_calculators::<T>.in_set(TickingSystemSet::PreTicking),
                destory_calculators::<T>.in_set(EndOfFrameSystemSet::PostTimerClearing),
            ),
        );
    }
}

pub fn initialize_calculators<T: Numeric>(
    mut calculator_event_channel_reader: EventReader<ValueCalculatorRequest>,
    mut timer_value_calculators: Query<&mut GoingEventValueCalculator<T>>,
) {
    for calculator_to_initialize in read_single_field_variant!(
        calculator_event_channel_reader,
        ValueCalculatorRequest::Initialize
    ) {
        if let Ok(mut value_calculator) = timer_value_calculators.get_mut(*calculator_to_initialize)
        {
            value_calculator.initialize_calculator();
        }
    }
}

pub fn destory_calculators<T: Numeric>(
    mut calculator_event_channel_reader: EventReader<ValueCalculatorRequest>,
    mut commands: Commands,
) {
    for calculator_to_destroy in read_single_field_variant!(
        calculator_event_channel_reader,
        ValueCalculatorRequest::Destroy
    ) {
        despawn_entity_notify_on_fail(
            *calculator_to_destroy,
            "an EmittingTimer's ValueCalculator",
            &mut commands,
        );
    }
}
