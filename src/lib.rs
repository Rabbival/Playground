#![allow(clippy::type_complexity)]
pub mod animation;
mod app;
mod common_logic;
mod data_structures;
mod debug;
pub mod ecs;
pub mod game;
mod input;
mod os_access;
pub mod time;
mod trait_unions;

#[macro_use]
mod macros;

#[macro_use]
extern crate lazy_static;

pub mod prelude {
    pub use crate::animation::{translation_change::*, CustomAnimationPlugin};
    pub use crate::app::{
        consts::*, generic_plugins::*, main, main_camera::*, screen_setup::*, tags::*,
    };
    pub use crate::common_logic::{
        argument_validation::*,
        enums::basic_direction::*,
        interpolator::{value_by_interpolation::*, Interpolator},
        mismatch_error::*,
        movement_type::*,
    };
    pub use crate::data_structures::vec_based_array::{vec_based_array_error::*, VecBasedArray};
    pub use crate::debug::{
        consts::*,
        enums::{bevy_log_level::*, log_category::*, os_access_log::*},
        game_session_log::*,
        print_log::*,
    };
    pub use crate::ecs::{
        component_utilities::*, entity_error::*, late_despawner::*, system_sets::*,
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
        emitting_timer::*,
        events::{
            calculate_and_send_going_event::*, remove_from_timer_affected_entities::*,
            set_time_multiplier::*, timer_done_event::*, timer_fire_request::*,
            timer_going_event::*, update_affected_entity_after_timer_birth::*,
            TimeEventChannelPlugin,
        },
        going_event_management::{going_event_emitting::*, going_event_value_calculator::*},
        time_multiplication::{
            time_multiplier::*, time_multiplier_id::*, time_multiplier_plugin::*,
            TimeMutiplicationPlugin,
        },
        time_related_error::*,
        timer_affected_entity::*,
        timer_calculators::{
            affecting_timer_calculators::*, affecting_timer_calculators_plugin::*,
            timer_calculator_set_policy::*,
        },
        timer_management::{
            timer_affected_entities_change::*, timer_clearing::*, timer_firing::*,
            timer_ticking::*, TimerManagementPlugin,
        },
        TimePlugin,
    };
    pub use crate::trait_unions::*;
    pub use bevy::{prelude::*, utils::HashMap};
    pub use std::marker::PhantomData;
}
