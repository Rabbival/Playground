use crate::prelude::*;

pub struct TimerFiringPlugin;

impl Plugin for TimerFiringPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                listen_for_full_timer_firing_requests::<MoveTimerFireRequest>,
                listen_for_full_timer_firing_requests::<TimeMultiplierChangeTimerFireRequest>,
            )
                .in_set(TimerSystemSet::PreTicking),
        );
    }
}

pub fn listen_for_full_timer_firing_requests<T: SendableTimerFireRequestType>(
    mut event_reader: EventReader<FullTimerFireRequest<T>>,
    mut remove_from_timer_entities_writer: EventWriter<RemoveFromTimerAffectedEntities>,
    mut full_timer_affected_entities: Query<&mut FullTimerAffected>,
    mut commands: Commands,
) {
    for timer_fire_request in event_reader.read() {
        //TODO: fix the following line once all there are multiple calculators per timer
        let event_entity = timer_fire_request.timer_to_fire.entities().array[0].unwrap();
        match full_timer_affected_entities.get_mut(event_entity) {
            Ok(mut affecting_timers_component) => replace_timer_for_entity(
                &mut remove_from_timer_entities_writer,
                timer_fire_request,
                &mut affecting_timers_component,
                &mut commands,
            ),
            Err(_) => print_warning(
                EntityError::EntityNotInQuery(
                    "couldn't find entity in affecting timers component query",
                ),
                vec![LogCategory::RequestNotFulfilled],
            ),
        }
    }
}

fn replace_timer_for_entity<T: SendableTimerFireRequestType>(
    remove_from_timer_entities_writer: &mut EventWriter<RemoveFromTimerAffectedEntities>,
    timer_fire_request: &FullTimerFireRequest<T>,
    affecting_timers: &mut FullTimerAffected,
    commands: &mut Commands,
) {
    let new_timer_entity = timer_fire_request.timer_to_fire.spawn_timer(commands);
    let maybe_existing_timer_for_movement_type = affecting_timers.insert(
        timer_fire_request.timer_to_fire.timer_going_event_type(),
        new_timer_entity,
        timer_fire_request.affecting_timer_set_policy,
    );
    if let Some(existing_timer) = maybe_existing_timer_for_movement_type {
        remove_timer_by_policy(
            remove_from_timer_entities_writer,
            timer_fire_request,
            new_timer_entity,
            existing_timer,
        );
    }
}

fn remove_timer_by_policy<T: SendableTimerFireRequestType>(
    remove_from_timer_entities_writer: &mut EventWriter<RemoveFromTimerAffectedEntities>,
    timer_fire_request: &FullTimerFireRequest<T>,
    new_timer: Entity,
    existing_timer: Entity,
) {
    let maybe_timer_to_remove_from = match timer_fire_request.affecting_timer_set_policy {
        AffectingTimerSetPolicy::AlwaysTakeNew => Some(existing_timer),
        AffectingTimerSetPolicy::IgnoreNewIfAssigned => Some(new_timer),
    };
    if let Some(timer_to_remove_from) = maybe_timer_to_remove_from {
        remove_from_timer_entities_writer.send(RemoveFromTimerAffectedEntities {
            timer_entity: timer_to_remove_from,
            //TODO: fix the following line once all there are multiple calculators per timer
            entity_to_remove: timer_fire_request.timer_to_fire.entities().array[0].unwrap(),
        });
    }
}
