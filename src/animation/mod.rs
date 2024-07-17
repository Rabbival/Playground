use crate::prelude::*;

pub mod orb_animation;

pub struct CustomeAnimationPlugin;

impl Plugin for CustomeAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(OrbAnimationPlugin);
    }
}
