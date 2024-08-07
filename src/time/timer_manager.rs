use crate::prelude::*;

pub struct TimerManagerPlugin;

impl Plugin for TimerManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                (
                    tick_timers::<f32>,
                    tick_timers::<Vec2>,
                    tick_timers::<Vec3>,
                    tick_timers::<Quat>,
                )
                    .in_set(TimerSystemSet::TimerTicking),
                (
                    add_timers_to_entities::<f32>,
                    add_timers_to_entities::<Vec2>,
                    add_timers_to_entities::<Vec3>,
                    add_timers_to_entities::<Quat>,
                )
                    .in_set(TimerSystemSet::TimerAttachment),
                (
                    clear_timers::<f32>,
                    clear_timers::<Vec2>,
                    clear_timers::<Vec3>,
                    clear_timers::<Quat>,
                )
                    .in_set(EndOfFrameSystemSet::TimerClearing),
            ),
        );
    }
}

fn tick_timers<T: Numeric>(
    mut event_from_timer_writer: EventWriter<EventFromTimer<T>>,
    mut timers_not_on_multiplers: Query<(&mut CustomTimer<T>, Entity)>,
    time_multipliers: Query<&TimeMultiplier>,
    time: Res<Time>,
) {
    let time_delta = time.delta_seconds();
    for (mut timer, timer_entity) in &mut timers_not_on_multiplers {
        tick_and_send_timer_event(
            time_delta * calculate_time_multiplier(&time_multipliers, &timer),
            &mut timer,
            timer_entity,
            &mut event_from_timer_writer,
        );
    }
}

fn calculate_time_multiplier<T: Numeric>(
    time_multipliers: &Query<&TimeMultiplier>,
    timer: &CustomTimer<T>,
) -> f32 {
    let mut calculated_multiplier = DEFAULT_TIME_MULTIPLIER;
    for maybe_multiplier_id in timer.time_multipliers {
        match maybe_multiplier_id {
            Some(multiplier_id_from_timer) => {
                for time_multiplier in time_multipliers {
                    if time_multiplier.id() == multiplier_id_from_timer {
                        calculated_multiplier *= time_multiplier.value();
                    }
                }
            }
            None => break,
        }
    }
    calculated_multiplier
}

fn tick_and_send_timer_event<T: Numeric>(
    time_to_tick: f32,
    timer: &mut CustomTimer<T>,
    timer_entity: Entity,
    event_from_timer_writer: &mut EventWriter<EventFromTimer<T>>,
) {
    if let Some(partial_timer_event) = timer.tick_and_get_event(time_to_tick) {
        event_from_timer_writer.send(EventFromTimer::<T>::from_partial(
            timer_entity,
            partial_timer_event,
        ));
    }
}

fn add_timers_to_entities<T: Numeric>(
    mut event_reader: EventReader<AddTimerToEntity<T>>,
    mut commands: Commands,
) {
    for timer_attachment_request in event_reader.read() {
        commands
            .entity(timer_attachment_request.entity)
            .insert(timer_attachment_request.timer);
    }
}

fn clear_timers<T: Numeric>(
    mut event_from_timer_reader: EventReader<EventFromTimer<T>>,
    mut commands: Commands,
) {
    for event_from_timer in event_from_timer_reader.read() {
        if let Some(done_event) = event_from_timer.try_get_done_event() {
            if let EventFromTimerType::DespawnSelf = done_event {
                commands.entity(event_from_timer.entity()).despawn();
            } else {
                commands
                    .entity(event_from_timer.entity())
                    .remove::<CustomTimer<Vec3>>();
            }
        }
    }
}
