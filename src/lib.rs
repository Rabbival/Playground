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
    pub use crate::common_logic::{argument_validation::*, enums::basic_direction::*};
    pub use crate::debug::{
        consts::*,
        errors::{mismatch_error::*, system_access_error::*},
        logs::{
            enums::{bevy_log_level::*, log_category::*, os_access_log::*},
            game_session_log::*,
            print_log::*,
        },
    };
    pub use crate::event_channels::{game_event_channels::*, timer_event::*, EventChannelPlugin};
    pub use crate::game::{consts::*, tags::*};
    pub use crate::input::{keyboard_input_handler::*, mouse_input_handler::*, InputPlugin};
    pub use crate::os_access::{
        enums::{folder_to_access::*, system_file_type::*},
        folder_access::*,
        system_file_name::*,
        text_file_access::*,
    };
    pub use crate::time::{
        consts::*,
        custom_timer::*,
        enums::{time_processor_id::*, timer_event_type::*},
        time_processor::*,
        time_processors::*,
        timer_manager::*,
        TimePlugin,
    };
    pub use crate::trait_unions::*;
    pub use bevy::{prelude::*, utils::HashMap};
}
