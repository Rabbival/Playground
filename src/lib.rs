#![allow(clippy::type_complexity)]
mod animation;
mod app;
mod common_logic;
mod debug;
mod event_channels;
mod game;
mod input;
mod os_access;
mod time;
mod trait_unions;

#[macro_use]
mod macros;

#[macro_use]
extern crate lazy_static;

pub mod prelude {
    pub use crate::animation::{orb_animation::*, CustomAnimationPlugin};
    pub use crate::app::{
        consts::*, main, main_camera::*, screen_setup::*, system_sets::*, tags::*,
    };
    pub use crate::common_logic::{
        argument_validation::*, enums::basic_direction::*, mismatch_error::*,
    };
    pub use crate::debug::{
        consts::*,
        enums::{bevy_log_level::*, log_category::*, os_access_log::*},
        game_session_log::*,
        print_log::*,
    };
    pub use crate::event_channels::{
        game_event_channels::*,
        timer_event_channel::{
            event_from_timer::*, time_processors_request::*, TimerEventChannel,
            TimerEventChannelPlugin,
        },
        EventChannelPlugin,
    };
    pub use crate::game::{consts::*, tags::*};
    pub use crate::input::{keyboard_input_handler::*, mouse_input_handler::*, InputPlugin};
    pub use crate::os_access::{
        enums::{folder_to_access::*, system_file_type::*},
        folder_access::*,
        os_access_error::*,
        system_file_name::*,
        text_file_access::*,
    };
    pub use crate::time::{
        consts::*,
        custom_timer::*,
        enums::{event_from_timer_type::*, time_processor_id::*},
        time_processor::*,
        time_processors::*,
        time_related_error::*,
        timer_manager::*,
        TimePlugin,
    };
    pub use crate::trait_unions::*;
    pub use bevy::{prelude::*, utils::HashMap};
}
