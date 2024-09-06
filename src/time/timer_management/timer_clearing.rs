use crate::{plugin_for_implementors_of_trait, prelude::*};

#[derive(Resource, Default)]
struct EmittingTimersDespawnedThisFrame(pub Vec<Entity>);

plugin_for_implementors_of_trait!(TimerClearingGenericPlugin, Numeric);

impl<T: Numeric> Plugin for TimerClearingGenericPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            listen_for_timer_done_events::<T>.in_set(EndOfFrameSystemSet::TimerClearing),
        );
    }
}

pub struct TimerClearingPlugin;

impl Plugin for TimerClearingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EmittingTimersDespawnedThisFrame>();
    }
}

fn listen_for_timer_done_events<T: Numeric>(
    mut timer_done_event_reader: EventReader<TimerDoneEvent>,
    emitting_timers: Query<(Entity, &EmittingTimer)>,
    mut emitting_timers_despawned_this_frame: ResMut<EmittingTimersDespawnedThisFrame>,
    mut commands: Commands,
) {
    for timer_done_event in timer_done_event_reader.read() {
        if let Ok((timer_entity, timer)) = emitting_timers.get(timer_done_event.timer_entity) {
            if !emitting_timers_despawned_this_frame
                .0
                .contains(&timer_entity)
            {
                despawn_entity_notify_on_fail(timer_entity, "EmittingTimer", &mut commands);
                emitting_timers_despawned_this_frame.0.push(timer_entity);
            }
            for value_calculator_entity in timer.calculator_entities_iter() {
                despawn_entity_notify_on_fail(
                    value_calculator_entity,
                    "a EmittingTimer's ValueCalculator",
                    &mut commands,
                );
            }
        }
    }
    emitting_timers_despawned_this_frame.0 = vec![];
}
