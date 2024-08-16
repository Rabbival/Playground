use crate::{get_mut_entity_else_return, prelude::*};

pub struct TranslationChangePlugin;

impl Plugin for TranslationChangePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                listen_for_init_translation_change_request.in_set(TimerSystemSet::PreTicking),
                listen_for_translation_update_requests.in_set(TimerSystemSet::PostTicking),
            ),
        );
    }
}

pub fn listen_for_init_translation_change_request(
    mut event_reader: EventReader<TranslationEventChannel>,
    mut remove_from_timer_entities_writer: EventWriter<RemoveFromTimerAffectedEntities>,
    mut full_timer_affected_entities: Query<&mut FullTimerAffected>,
    mut commands: Commands,
) {
    for translation_event in event_reader.read() {
        let event_entity = translation_event.event_entity();
        match full_timer_affected_entities.get_mut(event_entity) {
            Ok(mut affecting_timers_component) => replace_timer_for_entity(
                &mut affecting_timers_component.affecting_timers,
                &mut remove_from_timer_entities_writer,
                translation_event,
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

fn replace_timer_for_entity(
    entity_affecting_timers: &mut HashMap<TimerGoingEventType, Entity>,
    remove_from_timer_entities_writer: &mut EventWriter<RemoveFromTimerAffectedEntities>,
    translation_event: &TranslationEventChannel,
    commands: &mut Commands,
) {
    let event_type = translation_event.timer_going_event_type();
    let new_timer_entity = fire_timer_translation_change_timer(translation_event, commands);
    let maybe_existing_timer_for_movement_type =
        entity_affecting_timers.insert(event_type, new_timer_entity);
    if let Some(existing_timer) = maybe_existing_timer_for_movement_type {
        remove_from_timer_entities_writer.send(RemoveFromTimerAffectedEntities {
            timer_entity: existing_timer,
            entity_to_remove: translation_event.event_entity(),
        });
    }
}

fn fire_timer_translation_change_timer(
    translation_event: &TranslationEventChannel,
    commands: &mut Commands,
) -> Entity {
    match translation_event {
        TranslationEventChannel::InitiateMoveInDirectLine {
            entity,
            origin,
            target,
            duration,
            once_done,
        } => commands
            .spawn(CalculatingTimer {
                timer: FullTimer::new(
                    vec![*entity],
                    vec![TimeMultiplierId::GameTimeMultiplier],
                    *duration,
                    translation_event.timer_going_event_type(),
                    *once_done,
                ),
                calculator: ValueByInterpolation::<Vec3>::new(
                    *origin,
                    *target,
                    Interpolator {
                        power: ORB_COLLECTION_POWER,
                    },
                ),
            })
            .id(),
    }
}

fn listen_for_translation_update_requests(
    mut event_reader: EventReader<TimerGoingEvent<Vec3>>,
    mut transforms: Query<&mut Transform>,
) {
    for event_from_timer in event_reader.read() {
        if let TimerGoingEventType::Move(MoveEventFromTimer::InDirectLine) =
            event_from_timer.event_type
        {
            for entity in event_from_timer.entities.iter() {
                update_entity_translation(entity, &mut transforms, event_from_timer.value);
            }
        }
    }
}

fn update_entity_translation(
    entity: Entity,
    transforms: &mut Query<&mut Transform>,
    new_translation: Vec3,
) {
    let mut transform = get_mut_entity_else_return!(transforms, entity);
    transform.translation = new_translation;
}
