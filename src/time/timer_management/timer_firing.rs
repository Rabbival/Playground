use crate::{plugin_for_implementors_of_trait, prelude::*};

plugin_for_implementors_of_trait!(TimerFiringPlugin, SendableTimerFireRequestType);

impl<T: SendableTimerFireRequestType> Plugin for TimerFiringPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            listen_for_full_timer_firing_requests::<T>
                .in_set(TimerSystemSet::PreTickingPreperations),
        );
    }
}

pub fn listen_for_full_timer_firing_requests<T: SendableTimerFireRequestType>(
    mut event_reader: EventReader<FullTimerFireRequest<T>>,
    mut remove_from_timer_entities_writer: EventWriter<RemoveFromTimerAffectedEntities>,
    mut full_timer_affected_entities: Query<&mut AffectingTimers>,
    mut commands: Commands,
) {
    for timer_fire_request in event_reader.read() {
        let new_timer_entity = timer_fire_request
            .timer_firing_request
            .spawn_timer(&mut commands);
        for timer_affected_entity in timer_fire_request.timer_firing_request.entities().iter() {
            match full_timer_affected_entities.get_mut(timer_affected_entity.affected_entity) {
                Ok(mut affecting_timers_component) => replace_timer_for_entity(
                    &mut remove_from_timer_entities_writer,
                    timer_fire_request,
                    &mut affecting_timers_component,
                    new_timer_entity,
                    timer_affected_entity.affected_entity,
                ),
                Err(_) => print_warning(
                    EntityError::EntityNotInQuery(String::from(
                        "couldn't find entity in affecting timers component query",
                    )),
                    vec![LogCategory::RequestNotFulfilled],
                ),
            }
        }
    }
}

fn replace_timer_for_entity<T: SendableTimerFireRequestType>(
    remove_from_timer_entities_writer: &mut EventWriter<RemoveFromTimerAffectedEntities>,
    timer_fire_request: &FullTimerFireRequest<T>,
    affecting_timers: &mut AffectingTimers,
    new_timer_entity: Entity,
    affected_entity: Entity,
) {
    let maybe_existing_timer_for_movement_type = affecting_timers.insert(
        timer_fire_request
            .timer_firing_request
            .timer_going_event_type(),
        new_timer_entity,
        timer_fire_request.affecting_timer_set_policy,
    );
    if let Some(existing_timer) = maybe_existing_timer_for_movement_type {
        remove_timer_by_policy(
            remove_from_timer_entities_writer,
            timer_fire_request,
            new_timer_entity,
            existing_timer,
            affected_entity,
        );
    }
}

fn remove_timer_by_policy<T: SendableTimerFireRequestType>(
    remove_from_timer_entities_writer: &mut EventWriter<RemoveFromTimerAffectedEntities>,
    timer_fire_request: &FullTimerFireRequest<T>,
    new_timer: Entity,
    existing_timer: Entity,
    affected_entity: Entity,
) {
    let maybe_timer_to_remove_from = match timer_fire_request.affecting_timer_set_policy {
        AffectingTimerSetPolicy::AlwaysTakeNew => Some(existing_timer),
        AffectingTimerSetPolicy::IgnoreNewIfAssigned => Some(new_timer),
    };
    if let Some(timer_to_remove_from) = maybe_timer_to_remove_from {
        remove_from_timer_entities_writer.send(RemoveFromTimerAffectedEntities {
            timer_entity: timer_to_remove_from,
            entity_to_remove: affected_entity,
        });
    }
}
