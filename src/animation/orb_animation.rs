use crate::prelude::*;
use crate::{read_variant, return_if_at_limit};
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

pub struct OrbAnimationPlugin;

impl Plugin for OrbAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_orb);
    }
}

fn spawn_orb(
    mut spawn_request_reader: EventReader<SpawnRequest>,
    orb_query: Query<&Orb>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    return_if_at_limit!(orb_query, ORB_MAX_COUNT);
    for requested_spawn_location in read_variant!(spawn_request_reader, SpawnRequest::SpawnOrb) {
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Circle::new(ORB_MAX_RADIUS))),
                material: materials.add(Color::srgb(0.6, 0.1, 0.0)),
                transform: Transform::from_xyz(
                    requested_spawn_location.x,
                    requested_spawn_location.y,
                    0.0,
                ),
                ..default()
            },
            Orb,
        ));
    }
}
