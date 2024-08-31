use crate::{
    prelude::*, read_single_field_variant, return_if_at_limit,
    time::events::full_timer_fire_request::move_timer_fire_request::MoveTimerFireRequest,
};
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

pub fn spawn_orb(
    mut spawn_request_reader: EventReader<OrbEvent>,
    orb_query: Query<&Orb>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    for requested_spawn_location in
        read_single_field_variant!(spawn_request_reader, OrbEvent::SpawnOrb)
    {
        return_if_at_limit!(orb_query, ORB_MAX_COUNT);
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
            AffectingTimers::default(),
            Orb,
        ));
    }
}

pub fn collect_all_orbs(
    mut event_reader: EventReader<OrbEvent>,
    mut event_writer: EventWriter<FullTimerFireRequest<MoveTimerFireRequest>>,
    orb_query: Query<(&Transform, Entity), With<Orb>>,
    mut commands: Commands,
) {
    for orb_collection_target in read_single_field_variant!(event_reader, OrbEvent::CollectAllOrbs)
    {
        let mut orbs_and_interpolators = vec![];
        for (orb_transform, orb_entity) in &orb_query {
            let interpolator_id = commands
                .spawn(ValueByInterpolation::new(
                    orb_transform.translation,
                    Vec3::from((*orb_collection_target, 0.0)),
                    Interpolator::new(ORB_COLLECTION_POWER),
                ))
                .id();
            orbs_and_interpolators.push(FullTimerAffectedEntity {
                affected_entity: orb_entity,
                value_calculator_entity: interpolator_id,
            });
        }
        event_writer.send(FullTimerFireRequest {
            affecting_timer_set_policy: AffectingTimerSetPolicy::IgnoreNewIfAssigned,
            timer_firing_request: MoveTimerFireRequest::new(
                MovementType::InDirectLine,
                orbs_and_interpolators,
                vec![TimeMultiplierId::GameTimeMultiplier],
                ORB_COLLECTION_TIME,
                TimerDoneEventType::DespawnAffectedEntities,
            ),
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_orb_limitation() {
        let mut app = App::new();
        let more_than_max = ORB_MAX_COUNT + 1;
        app.init_resource::<Assets<Mesh>>()
            .init_resource::<Assets<ColorMaterial>>()
            .add_event::<OrbEvent>()
            .add_systems(Update, (send_orb_event, spawn_orb).chain());

        for _ in 0..more_than_max {
            app.update();
        }

        assert_eq!(
            app.world_mut().query::<&Orb>().iter(app.world()).len(),
            ORB_MAX_COUNT
        );
    }

    fn send_orb_event(mut event_writer: EventWriter<OrbEvent>) {
        event_writer.send(OrbEvent::SpawnOrb(Vec2::default()));
    }
}
