use std::time::Duration;

use playground::prelude::*;

mod test_dependencies;

#[test]
fn orb_spawn_and_despawn_test() {
    let mut app = test_dependencies::get_app_with_resources_and_events();
    app.add_systems(
        Update,
        (
            spawn_orb,
            collect_all_orbs,
            listen_for_init_translation_change_request,
            tick_full_timers::<Vec3>,
            listen_for_despawn_requests_from_timers,
        )
            .chain(),
    );
    test_dependencies::request_max_orb_spawns(&mut app);
    app.world_mut()
        .resource_mut::<Events<OrbEvent>>()
        .send(OrbEvent::CollectAllOrbs(Vec2::default()));

    app.update();
    let orb_count_before_time_passed = count_orbs_in_world(&mut app);
    app.world_mut()
        .resource_mut::<Time>()
        .as_mut()
        .advance_by(Duration::from_secs_f32(ORB_COLLECTION_TIME));
    app.update();

    assert_eq!(orb_count_before_time_passed, ORB_MAX_COUNT);
    assert_eq!(count_orbs_in_world(&mut app), 0);
}

fn count_orbs_in_world(app: &mut App) -> usize {
    app.world_mut().query::<&Orb>().iter(app.world()).len()
}
