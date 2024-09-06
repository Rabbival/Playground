use crate::prelude::*;

#[derive(Event, Debug, Clone, Copy)]
pub struct TimerFireRequest(pub EmittingTimer);

pub struct TimerFireRequestPlugin;

impl Plugin for TimerFireRequestPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TimerFireRequest>();
    }
}
