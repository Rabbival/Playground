#![allow(clippy::type_complexity)]
mod animation;
mod app;
mod camera;
mod consts;
mod debug;
mod event_channels;
mod input;
mod logic_enums;
mod screen_setup;
mod tags;

#[macro_use]
mod my_macros;

pub mod prelude {
    pub use crate::animation::{orb_animation::*, CustomeAnimationPlugin};
    pub use crate::app::*;
    pub use crate::camera::*;
    pub use crate::consts::{debug_consts::*, game_consts::*};
    pub use crate::debug::{debug_print::*, enums::debug_print_type::*, game_session_log::*};
    pub use crate::event_channels::{game_event_channels::*, EventChannelPlugin};
    pub use crate::input::{keyboard_input_handler::*, mouse_input_handler::*, CostumeInputPlugin};
    pub use crate::logic_enums::basic_direction::*;
    pub use crate::screen_setup::*;
    pub use crate::tags::game_tags::*;
    pub use bevy::{prelude::*, utils::HashMap};
}
