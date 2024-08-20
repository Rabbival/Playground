use crate::prelude::*;

pub mod full_timer_fire_request_type;
pub mod move_timer_fire_request;
pub mod time_multiplier_change_timer_fire_request;

#[derive(Event, Debug, Clone, Copy)]
pub struct FullTimerFireRequest<T: SendableTimerFireRequestType> {
    pub affecting_timer_set_policy: AffectingTimerSetPolicy,
    pub timer_to_fire: T,
}

pub struct FullTimerFireRequestPlugin;

impl Plugin for FullTimerFireRequestPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<FullTimerFireRequest<MoveTimerFireRequest>>()
            .add_event::<FullTimerFireRequest<TimeMultiplierChangeTimerFireRequest>>();
    }
}
