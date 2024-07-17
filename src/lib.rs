#![allow(clippy::type_complexity)]
mod animation;
mod app;
mod camera;
mod consts;
mod enums;
mod event_channels;
mod input;
mod screen_setup;
mod tags;

#[macro_use]
mod my_macros;

pub mod prelude {
    pub use crate::animation::{orb_animation::*, CustomeAnimationPlugin};
    pub use crate::app::*;
    pub use crate::camera::*;
    pub use crate::consts::game_consts::*;
    pub use crate::enums::basic_direction::*;
    pub use crate::event_channels::{game_event_channels::*, EventChannelPlugin};
    pub use crate::input::{keyboard_input_handler::*, mouse_input_handler::*, CostumeInputPlugin};
    pub use crate::screen_setup::*;
    pub use crate::tags::game_tags::*;
    pub use bevy::{prelude::*, utils::HashMap};
}
