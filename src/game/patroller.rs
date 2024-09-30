use crate::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

pub struct PatrollerPlugin;

impl Plugin for PatrollerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (spawn_patroller, initiate_patroller_movement).chain(),
        );
    }
}

pub fn spawn_patroller(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Circle::new(ORB_MAX_RADIUS))),
            material: materials.add(Color::srgb(0.5, 0.0, 0.5)),
            transform: Transform::from_xyz(
                WINDOW_SIZE_IN_PIXELS / 8.0,
                WINDOW_SIZE_IN_PIXELS / 8.0,
                0.0,
            ),
            ..default()
        },
        AffectingTimerCalculators::default(),
        Patroller,
    ));
}

pub fn initiate_patroller_movement(
    mut event_writer: EventWriter<TimerFireRequest>,
    patroller_query: Query<(Entity, &Transform), With<Patroller>>,
    mut commands: Commands,
) {
    for (patroller_entity, patroller_transform) in &patroller_query {
        let all_path_vertices = determine_all_path_vertices(
            patroller_transform.translation,
            &mut vec![Vec3::new(-50.0, 0.0, 0.0)],
            true,
        );
        let going_event_value_calculators =
            configure_value_calculators_for_patroller(all_path_vertices, 2.0);
        let mut emitting_timers = vec![];
        for value_calculator in going_event_value_calculators {
            spawn_calculator_and_push_timer(
                patroller_entity,
                value_calculator,
                &mut emitting_timers,
                &mut commands,
            );
        }
        if let Err(timer_sequence_error) = TimerSequence::spawn_sequence_and_fire_first_timer(
            &mut event_writer,
            &emitting_timers,
            true,
            &mut commands,
        ) {
            print_error(
                timer_sequence_error,
                vec![LogCategory::Time, LogCategory::RequestNotFulfilled],
            );
        }
    }
}

fn spawn_calculator_and_push_timer(
    patroller_entity: Entity,
    value_calculator: GoingEventValueCalculator<Vec3>,
    emitting_timers: &mut Vec<EmittingTimer>,
    commands: &mut Commands,
) {
    let value_calculator_id = commands.spawn(value_calculator).id();
    emitting_timers.push(EmittingTimer::new(
        vec![TimerAffectedEntity {
            affected_entity: patroller_entity,
            value_calculator_entity: Some(value_calculator_id),
        }],
        vec![TimeMultiplierId::GameTimeMultiplier],
        EXAMPLE_PATROLLER_DURATION,
        TimerDoneEventType::Nothing,
    ));
}

fn determine_all_path_vertices(
    patroller_translation: Vec3,
    destinations: &mut Vec<Vec3>,
    go_to_start_position_once_done: bool,
) -> Vec<Vec3> {
    let mut all_path_vertices = vec![patroller_translation];
    all_path_vertices.append(destinations);
    if go_to_start_position_once_done {
        all_path_vertices.push(patroller_translation);
    }
    all_path_vertices
}

fn configure_value_calculators_for_patroller(
    all_path_vertices: Vec<Vec3>,
    interpolator_power: f32,
) -> Vec<GoingEventValueCalculator<Vec3>> {
    let mut value_calculators = vec![];
    for (index, vertice) in all_path_vertices.iter().enumerate() {
        value_calculators.push(GoingEventValueCalculator::new(
            TimerCalculatorSetPolicy::IgnoreNewIfAssigned,
            ValueByInterpolation::from_goal_and_current(
                *vertice,
                *all_path_vertices
                    .get(index + 1)
                    .unwrap_or(all_path_vertices.first().unwrap()), //if it's empty we wouldn't get in the for loop
                Interpolator::new(interpolator_power),
            ),
            TimerGoingEventType::Move(MovementType::InDirectLine),
        ));
    }
    value_calculators
}
