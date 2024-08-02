use crate::prelude::*;

pub mod orb_animation;
pub mod translation_change;

pub struct CustomAnimationPlugin;

impl Plugin for CustomAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((OrbAnimationPlugin, TranslationChangePlugin));
    }
}
