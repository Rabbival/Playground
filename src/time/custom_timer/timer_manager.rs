use crate::prelude::*;

pub struct TimerManagerPlugin;

impl Plugin for TimerManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                (
                    tick_once_done_timers,
                    tick_full_timers::<f32>,
                    tick_full_timers::<Vec2>,
                    tick_full_timers::<Vec3>,
                    tick_full_timers::<Quat>,
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

fn tick_once_done_timers(
    mut timer_done_event_writer: EventWriter<TimerDoneEvent>,
    mut timers: Query<(&mut OnceDoneTimer, Entity)>,
    time_multipliers: Query<&TimeMultiplier>,
    time: Res<Time>,
) {
    let time_delta = time.delta_seconds();
    for (mut timer, timer_entity) in &mut timers {}
}

fn tick_full_timers<T: Numeric>(
    mut timer_going_event_writer: EventWriter<TimerGoingEvent<T>>,
    mut timer_done_event_writer: EventWriter<TimerDoneEvent>,
    mut timers: Query<(&mut FullTimer, &ValueByInterpolation<T>, Entity)>,
    time_multipliers: Query<&TimeMultiplier>,
    time: Res<Time>,
) {
    let time_delta = time.delta_seconds();
    for (mut timer, value_by_interpolation, timer_entity) in &mut timers {
        tick_and_send_timer_event(
            time_delta * calculate_time_multiplier(&time_multipliers, &timer),
            &mut timer,
            timer_entity,
            &mut event_from_timer_writer,
        );
    }
}

fn calculate_time_multiplier(time_multipliers: &Query<&TimeMultiplier>, timer: &FullTimer) -> f32 {
    let mut calculated_multiplier = DEFAULT_TIME_MULTIPLIER;
    for multiplier_id_from_timer in timer.time_multipliers.iter() {
        for time_multiplier in time_multipliers {
            if time_multiplier.id() == multiplier_id_from_timer {
                calculated_multiplier *= time_multiplier.value();
            }
        }
    }
    calculated_multiplier
}

fn tick_and_send_timer_event<T: Numeric>(
    time_to_tick: f32,
    timer: &mut FullTimer<T>,
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
    mut timer_done_event_reader: EventReader<TimerDoneEvent>,
    mut commands: Commands,
) {
    for timer_done_event in timer_done_event_reader.read() {
        let mut timer_entity_commands = commands.entity(timer_done_event.timer_entity);
        timer_entity_commands.remove::<CalculatingTimer<T>>();
        timer_entity_commands.remove::<OnceDoneTimer>();
    }
}
