use crate::{plugin_for_implementors_of_trait, prelude::*};

pub mod full_timer_fire_request_type;
pub mod move_timer_fire_request;
pub mod time_multiplier_change_timer_fire_request;

#[derive(Event, Debug, Clone, Copy)]
pub struct FullTimerFireRequest<T: SendableTimerFireRequestType> {
    pub affecting_timer_set_policy: AffectingTimerSetPolicy,
    pub timer_to_fire: T,
}

plugin_for_implementors_of_trait!(FullTimerFireRequestPlugin, SendableTimerFireRequestType);

impl<T: SendableTimerFireRequestType> Plugin for FullTimerFireRequestPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_event::<FullTimerFireRequest<T>>();
    }
}
