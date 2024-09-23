use crate::prelude::*;

pub struct TimerSequenceManagerPlugin;

impl Plugin for TimerSequenceManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, listen_for_fire_next_timer_requests);
    }
}

fn listen_for_fire_next_timer_requests() {}
