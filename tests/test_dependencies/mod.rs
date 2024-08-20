use playground::prelude::*;

pub fn get_app_with_resources_and_events() -> App {
    let mut app = App::new();
    app.init_resource::<Time>()
        .init_resource::<Assets<Mesh>>()
        .init_resource::<Assets<ColorMaterial>>()
        .add_event::<TimerDoneEvent>()
        .add_event::<OrbEvent>()
        .add_event::<TimerDoneEvent>()
        .add_event::<TimerGoingEvent<f32>>()
        .add_event::<TimerGoingEvent<Vec2>>()
        .add_event::<TimerGoingEvent<Vec3>>()
        .add_event::<TimerGoingEvent<Quat>>()
        .add_event::<FullTimerFireRequest<MoveTimerFireRequest>>()
        .add_event::<RemoveFromTimerAffectedEntities>();
    app
}

pub fn request_max_orb_spawns(app: &mut App) {
    let mut orb_event_channel_writer = app.world_mut().resource_mut::<Events<OrbEvent>>();
    for _ in 0..ORB_MAX_COUNT {
        orb_event_channel_writer.send(OrbEvent::SpawnOrb(Vec2::default()));
    }
}
