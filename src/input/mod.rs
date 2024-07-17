use crate::prelude::*;

pub mod keyboard_input_handler;
pub mod mouse_input_handler;

pub struct CostumeInputPlugin;

impl Plugin for CostumeInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((KeyboardInputHandlerPlugin, MouseInputHandlerPlugin));
    }
}
