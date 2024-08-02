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
    pub use crate::animation::{CustomAnimationPlugin, translation_change::*, event_channels::*};
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
    pub use crate::game::{consts::*, tags::*, GamePlugin, orb::*, event_channels::*};
    pub use crate::input::{keyboard_input_handler::*, mouse_input_handler::*, InputPlugin};
    pub use crate::os_access::{
        enums::{folder_to_access::*, system_file_type::*},
        folder_access::*,
        os_access_error::*,
        system_file_name::*,
        text_file_access::*,
    };
    pub use crate::time::{
        event_channel::{
            event_from_timer::*, time_processors_request::*, TimerEventChannel,
            TimerEventChannelPlugin, event_from_timer_type::*,
        },
        consts::*,
        custom_timer::*,
        time_processing::{
            time_processor::*, time_processors::*, time_processors_update::*, TimeProcessingPlugin, time_processor_id::*
        },
        time_related_error::*,
        timer_manager::*,
        TimePlugin,
    };
    pub use crate::trait_unions::*;
    pub use bevy::{prelude::*, utils::HashMap};
}
