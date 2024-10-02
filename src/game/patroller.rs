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
        let all_path_vertices = PathTravelType::OneWay.apply_to_path(vec![
            patroller_transform.translation,
            Vec3::new(-50.0, 0.0, 0.0),
            Vec3::new(-100.0, 0.0, 0.0),
        ]);
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
            false, //true,
            &mut commands,
        ) {
            print_error(
                timer_sequence_error,
                vec![LogCategory::Time, LogCategory::RequestNotFulfilled],
            );
        }
    }
}

fn configure_value_calculators_for_patroller(
    all_path_vertices: Vec<Vec3>,
    interpolator_power: f32,
) -> Vec<GoingEventValueCalculator<Vec3>> {
    let mut value_calculators = vec![];
    let vertice_count = all_path_vertices.iter().len();
    for (index, vertice) in all_path_vertices.iter().enumerate() {
        if index == vertice_count - 1 {
            break;
        }
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
    print_info_vec("vertices: ", &all_path_vertices, vec![LogCategory::Time]);
    print_info_vec(
        "value calculators: ",
        &value_calculators,
        vec![LogCategory::Time],
    );
    value_calculators
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
