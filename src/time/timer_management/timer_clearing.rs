use crate::prelude::*;

#[derive(Resource, Default)]
pub struct EmittingTimersDespawnedThisFrame(pub Vec<Entity>);

pub struct TimerClearingPlugin;

impl Plugin for TimerClearingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EmittingTimersDespawnedThisFrame>()
            .add_systems(
                Update,
                (
                    clear_emitting_timer_despawned_this_frame
                        .in_set(EndOfFrameSystemSet::PreTimerClearing)
                        .run_if(
                            resource_exists_and_changed::<EmittingTimersDespawnedThisFrame>
                                .and_then(not(resource_added::<EmittingTimersDespawnedThisFrame>)),
                        ),
                    clear_done_timers.in_set(EndOfFrameSystemSet::TimerClearing),
                ),
            );
    }
}

pub fn clear_emitting_timer_despawned_this_frame(
    mut emitting_timers_despawned_this_frame: ResMut<EmittingTimersDespawnedThisFrame>,
) {
    let despawned_timers_vector = &emitting_timers_despawned_this_frame.0;
    if !despawned_timers_vector.is_empty() {
        emitting_timers_despawned_this_frame.0 = vec![];
    }
}

pub fn clear_done_timers(
    mut timer_done_event_reader: EventReader<TimerDoneEvent>,
    mut destroy_calculator_event_writer: EventWriter<ValueCalculatorRequest>,
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
            if timer_done_event.timer_parent_sequence.is_none() {
                for value_calculator_entity in timer.calculator_entities_iter() {
                    destroy_calculator_event_writer
                        .send(ValueCalculatorRequest::Destroy(value_calculator_entity));
                }
            }
        }
    }
}
