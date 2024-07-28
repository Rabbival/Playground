use crate::prelude::*;

pub mod orb_animation;

pub struct CustomAnimationPlugin;

impl Plugin for CustomAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(OrbAnimationPlugin);
    }
}
