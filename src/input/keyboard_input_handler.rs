use crate::prelude::*;

pub struct KeyboardInputHandlerPlugin;

impl Plugin for KeyboardInputHandlerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, listen_for_move_requests);
    }
}

fn listen_for_move_requests(keyboard_input: Res<ButtonInput<KeyCode>>) {
    let move_requests = keyboard_input
        .get_just_pressed()
        .map(BasicDirection::from_keycode);
    for _request in move_requests {
        //send an event
    }
}
