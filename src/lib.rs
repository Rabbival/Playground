#![allow(clippy::type_complexity)]
mod animation;
mod app;
mod camera;
mod consts;
mod debug;
mod event_channels;
mod input;
mod logic_enums;
mod os_access;
mod screen_setup;
mod tags;

#[macro_use]
mod my_macros;

#[macro_use]
extern crate lazy_static;

pub mod prelude {
    pub use crate::animation::{orb_animation::*, CustomeAnimationPlugin};
    pub use crate::app::*;
    pub use crate::camera::*;
    pub use crate::consts::{app_consts::*, debug_consts::*, game_consts::*};
    pub use crate::debug::{
        errors::{mismatch_error::*, system_access_error::*},
        logs::{
            enums::{bevy_log_level::*, log_category::*, os_access_log::*},
            game_session_log::*,
            print_log::*,
        },
    };
    pub use crate::event_channels::{game_event_channels::*, EventChannelPlugin};
    pub use crate::input::{keyboard_input_handler::*, mouse_input_handler::*, CostumeInputPlugin};
    pub use crate::logic_enums::{basic_direction::*, system_sets::*};
    pub use crate::os_access::{
        enums::{folder_to_access::*, system_file_type::*},
        folder_access::*,
        system_file_name::*,
        text_file_access::*,
    };
    pub use crate::screen_setup::*;
    pub use crate::tags::game_tags::*;
    pub use bevy::{prelude::*, utils::HashMap};
}
