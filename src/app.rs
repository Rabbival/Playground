#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use crate::prelude::*;

#[bevy_main]
pub fn main() {
    let mut app = App::new();
    app
        //bevy basics
        .add_plugins(ScreenSetupPlugin)
        //costume
        .add_plugins((
            CostumeInputPlugin,
            CameraPlugin,
            CustomeAnimationPlugin,
            EventChannelPlugin,
        ));

    if APPEND_DEBUG_MESSAGES_TO_LOG_FILE {
        app.add_plugins(GameSessionLogPlugin);
    }

    app.run();
}
