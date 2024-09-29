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

pub fn initiate_patroller_movement(patroller_query: Query<&Patroller>) {}
