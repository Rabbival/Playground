use crate::prelude::*;

pub mod full_timer_fire_request;
pub mod remove_from_timer_affected_entities;
pub mod set_time_multiplier;
pub mod timer_done_event;
pub mod timer_going_event;

pub struct TimeEventChannelPlugin;

impl Plugin for TimeEventChannelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            SetTimeMultiplierPlugin,
            TimerDoneEventPlugin,
            TimerGoingEventPlugin,
            FullTimerFireRequestPlugin,
            RemoveFromTimerAffectedEntitiesPlugin,
        ));
    }
}
