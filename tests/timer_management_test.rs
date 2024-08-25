use playground::prelude::*;

mod test_dependencies;

#[test]
fn timer_always_take_new_policy_test() {
    timer_policy_test(AffectingTimerSetPolicy::AlwaysTakeNew, 1);
}

#[test]
fn timer_ignore_new_if_assigned_policy_test() {
    timer_policy_test(AffectingTimerSetPolicy::IgnoreNewIfAssigned, 1);
}

fn timer_policy_test(policy: AffectingTimerSetPolicy, expected_entity_count_after_fire: usize) {
    const TIMER_DURATION_IN_SECONDS: f32 = 60.0;
    let affected_entities_when_single_timer;
    let affected_entities_after_new_timer_spawned;
    let mut app = test_dependencies::get_app_with_resources_and_events();
    let empty_entity = app
        .world_mut()
        .commands()
        .spawn(FullTimerAffected::default())
        .id();
    app.add_systems(
        Update,
        (
            listen_for_full_timer_firing_requests::<MoveTimerFireRequest>,
            listen_for_affected_entity_removal_request,
        )
            .chain(),
    );

    request_full_timer_firing(
        &mut app,
        AffectingTimerSetPolicy::default(),
        vec![empty_entity],
        TIMER_DURATION_IN_SECONDS,
    );
    app.update();
    affected_entities_when_single_timer = count_affected_entities(&mut app);
    request_full_timer_firing(
        &mut app,
        policy,
        vec![empty_entity],
        TIMER_DURATION_IN_SECONDS,
    );
    app.update();
    affected_entities_after_new_timer_spawned = count_affected_entities(&mut app);

    assert_eq!(affected_entities_when_single_timer, 1);
    assert_eq!(
        affected_entities_after_new_timer_spawned,
        expected_entity_count_after_fire
    );
}

fn request_full_timer_firing(
    app: &mut App,
    policy: AffectingTimerSetPolicy,
    affected_entities: Vec<Entity>,
    duration: f32,
) {
    app.world_mut()
        .resource_mut::<Events<FullTimerFireRequest<MoveTimerFireRequest>>>()
        .send(FullTimerFireRequest {
            affecting_timer_set_policy: policy,
            timer_firing_request: MoveTimerFireRequest::new(
                MovementType::default(),
                ValueByInterpolation::default(),
                affected_entities,
                vec![],
                duration,
                TimerDoneEventType::default(),
            ),
        });
}

fn count_affected_entities(app: &mut App) -> usize {
    let mut affected_entities_count = 0;
    for full_timer in app.world_mut().query::<&FullTimer>().iter(app.world()) {
        affected_entities_count += full_timer.affected_entities.len()
    }
    affected_entities_count
}
