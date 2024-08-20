use crate::prelude::*;

pub struct FullTimerAffectedPlugin;

impl Plugin for FullTimerAffectedPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            clear_done_timers_from_affecting_timers.in_set(TimerSystemSet::PostTickingInstant),
        );
    }
}

fn clear_done_timers_from_affecting_timers(
    mut timer_done_event_reader: EventReader<TimerDoneEvent>,
    mut timer_affected_entities: Query<&mut FullTimerAffected>,
    full_timers: Query<&FullTimer>,
) {
    for done_event in timer_done_event_reader.read() {
        let maybe_full_timer = full_timers.get(done_event.timer_entity);
        if let Ok(full_timer) = maybe_full_timer {
            for mut timer_affected in &mut timer_affected_entities {
                timer_affected.remove(&full_timer.send_as_going);
            }
        }
    }
}
