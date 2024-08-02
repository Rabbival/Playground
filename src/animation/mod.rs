use crate::prelude::*;

pub mod translation_change;
pub mod event_channels;

pub struct CustomAnimationPlugin;

impl Plugin for CustomAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((TranslationChangePlugin, AnimationEventChannelsPlugin));
    }
}
