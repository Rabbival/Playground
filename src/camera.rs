use crate::prelude::*;
use bevy::render::view::RenderLayers;

#[derive(Component)]
pub struct MainCamera;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        MainCamera,
        Camera2dBundle {
            camera: Camera::default(),
            ..default()
        },
        RenderLayers::layer(0),
    ));
}
