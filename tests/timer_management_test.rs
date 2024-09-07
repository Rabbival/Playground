use playground::prelude::*;

mod test_dependencies;

#[test]
fn timer_always_take_new_policy_test() {
    timer_policy_test(TimerCalculatorSetPolicy::AlwaysTakeNew, 1);
}

#[test]
fn timer_ignore_new_if_assigned_policy_test() {
    timer_policy_test(TimerCalculatorSetPolicy::IgnoreNewIfAssigned, 1);
}

fn timer_policy_test(policy: TimerCalculatorSetPolicy, expected_entity_count_after_fire: usize) {
    const TIMER_DURATION_IN_SECONDS: f32 = 60.0;
    let affected_entities_when_single_timer;
    let affected_entities_after_new_timer_spawned;
    let mut app = test_dependencies::get_app_with_resources_and_events();
    let redundant_calculator = app
        .world_mut()
        .commands()
        .spawn(GoingEventValueCalculator::new(
            policy,
            ValueByInterpolation::new(0.0, 0.0, Interpolator::default()),
            TimerGoingEventType::ChangeTimeMultiplierSpeed,
        ))
        .id();
    let empty_entity = app
        .world_mut()
        .commands()
        .spawn(AffectingTimerCalculators::default())
        .id();
    let redundant_affected_entity = TimerAffectedEntity {
        affected_entity: empty_entity,
        value_calculator_entity: Some(redundant_calculator),
    };

    app.add_systems(
        Update,
        (
            listen_for_emitting_timer_firing_requests,
            listen_for_update_affected_entities_after_timer_birth_requests::<f32>,
            listen_for_affected_entity_removal_request,
        )
            .chain(),
    );

    request_emitting_timer_firing(
        &mut app,
        redundant_affected_entity,
        TIMER_DURATION_IN_SECONDS,
    );
    app.update();
    affected_entities_when_single_timer = count_affected_entities(&mut app);
    request_emitting_timer_firing(
        &mut app,
        redundant_affected_entity,
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

fn request_emitting_timer_firing(
    app: &mut App,
    affected_entity: TimerAffectedEntity,
    duration: f32,
) {
    app.world_mut()
        .resource_mut::<Events<TimerFireRequest>>()
        .send(TimerFireRequest(EmittingTimer::new(
            vec![affected_entity],
            vec![],
            duration,
            TimerDoneEventType::default(),
        )));
}

fn count_affected_entities(app: &mut App) -> usize {
    let mut affected_entities_count = 0;
    for emitting_timer in app.world_mut().query::<&EmittingTimer>().iter(app.world()) {
        affected_entities_count += emitting_timer.affected_entities.len()
    }
    affected_entities_count
}
