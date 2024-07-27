#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use crate::prelude::*;

pub mod screen_setup;
pub mod system_sets;

#[bevy_main]
pub fn main() {
    let mut app = App::new();
    app
        //bevy basics
        .add_plugins(ScreenSetupPlugin)
        //costume
        .add_plugins((
            SystemSetsPlugin,
            CostumeInputPlugin,
            CameraPlugin,
            CustomeAnimationPlugin,
            EventChannelPlugin,
        ));

    if !LOG_CATEGORYS_TO_APPEND_TO_SESSION_LOG.is_empty() {
        app.add_plugins(GameSessionLogPlugin);
    }

    app.run();
}
