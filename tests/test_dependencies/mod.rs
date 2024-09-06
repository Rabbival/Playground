use std::time::Duration;

use playground::prelude::*;

pub fn get_app_with_resources_and_events() -> App {
    let mut app = App::new();
    app.init_resource::<Time>()
        .init_resource::<Assets<Mesh>>()
        .init_resource::<Assets<ColorMaterial>>()
        .add_event::<TimerDoneEvent>()
        .add_event::<OrbEvent>()
        .add_event::<TimerGoingEvent<f32>>()
        .add_event::<TimerGoingEvent<Vec2>>()
        .add_event::<TimerGoingEvent<Vec3>>()
        .add_event::<TimerGoingEvent<Quat>>()
        .add_event::<TimerFireRequest>()
        .add_event::<RemoveFromTimerAffectedEntities>()
        .add_event::<CalculateAndSendGoingEvent>();
    app
}

pub fn fast_forward(app: &mut App, time_to_advance_in_seconds: f32) {
    app.world_mut()
        .resource_mut::<Time>()
        .as_mut()
        .advance_by(Duration::from_secs_f32(time_to_advance_in_seconds));
}
