use crate::{prelude::*, read_single_field_variant, return_if_at_limit};
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

pub struct OrbPlugin;

impl Plugin for OrbPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (spawn_orb, collect_all_orbs).in_set(InputSystemSet::Handling),
        );
    }
}

fn spawn_orb(
    mut spawn_request_reader: EventReader<OrbEvent>,
    orb_query: Query<&Orb>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    return_if_at_limit!(orb_query, ORB_MAX_COUNT);
    for requested_spawn_location in
        read_single_field_variant!(spawn_request_reader, OrbEvent::SpawnOrb)
    {
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

fn collect_all_orbs(
    mut event_reader: EventReader<OrbEvent>,
    mut event_writer: EventWriter<TranslationEventChannel>,
    orb_query: Query<(&Transform, Entity), With<Orb>>,
) {
    for orb_collection_target in read_single_field_variant!(event_reader, OrbEvent::CollectAllOrbs)
    {
        for (orb_transform, orb_entity) in &orb_query {
            event_writer.send(TranslationEventChannel::MoveInDirectLine {
                entity: orb_entity,
                origin: orb_transform.translation,
                target: Vec3::from((*orb_collection_target, 0.0)),
                duration: ORB_COLLECTION_TIME,
                once_done: Some(EventFromTimerType::DespawnSelf),
            });
        }
    }
}
