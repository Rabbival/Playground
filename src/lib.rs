#![allow(clippy::type_complexity)]
mod animation;
mod app;
mod common_logic;
mod debug;
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
    pub use crate::animation::{event_channels::*, translation_change::*, CustomAnimationPlugin};
    pub use crate::app::{
        consts::*, main, main_camera::*, screen_setup::*, system_sets::*, tags::*,
    };
    pub use crate::common_logic::{
        argument_validation::*, enums::basic_direction::*, interpolator::*, mismatch_error::*,
    };
    pub use crate::debug::{
        consts::*,
        enums::{bevy_log_level::*, log_category::*, os_access_log::*},
        game_session_log::*,
        print_log::*,
    };
    pub use crate::game::{consts::*, event_channels::*, orb::*, tags::*, GamePlugin};
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
        event_channels::{
            event_from_timer::*, event_from_timer_type::*, TimeEventChannel, TimeEventChannelPlugin,
        },
        time_multiplication::{
            time_multiplier::*, time_multiplier_id::*, time_multiplier_plugin::*,
            TimeMutiplicationPlugin,
        },
        time_related_error::*,
        timer_manager::*,
        timer_value_calculator::*,
        TimePlugin,
    };
    pub use crate::trait_unions::*;
    pub use bevy::{prelude::*, utils::HashMap};
}
