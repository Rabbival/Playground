use crate::prelude::*;

pub struct TimerAffectedEntitiesChangePlugin;

impl Plugin for TimerAffectedEntitiesChangePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            listen_for_affected_entity_removal_request.in_set(TimerSystemSet::PreTicking),
        );
    }
}

pub fn listen_for_affected_entity_removal_request(
    mut event_reader: EventReader<RemoveFromTimerAffectedEntities>,
    mut once_done_timers: Query<&mut OnceDoneTimer>,
    mut full_timers: Query<&mut FullTimer>,
) {
    for removal_request in event_reader.read() {
        if let Err(time_related_error) =
            remove_affected_entity(removal_request, &mut once_done_timers, &mut full_timers)
        {
            print_error(time_related_error, vec![LogCategory::RequestNotFulfilled])
        } else {
            print_info(
                format!(
                    "Removed entity {:?} from timer: {:?}",
                    removal_request.entity_to_remove, removal_request.timer_entity
                ),
                vec![LogCategory::Time],
            );
        }
    }
}

fn remove_affected_entity(
    removal_request: &RemoveFromTimerAffectedEntities,
    once_done_timers: &mut Query<&mut OnceDoneTimer>,
    full_timers: &mut Query<&mut FullTimer>,
) -> Result<(), TimeRelatedError> {
    let timer_entity = removal_request.timer_entity;
    let entity_to_remove = removal_request.entity_to_remove;
    if let Ok(mut once_done_timer) = once_done_timers.get_mut(timer_entity) {
        once_done_timer
            .affected_entities
            .remove_by_item(entity_to_remove)?;
        Ok(())
    } else if let Ok(mut full_timer) = full_timers.get_mut(timer_entity) {
        full_timer
            .affected_entities
            .remove_by_affected_entity(entity_to_remove)?;
        Ok(())
    } else {
        Err(TimeRelatedError::TimerToRemoveFromNotFound(
            *removal_request,
        ))
    }
}
