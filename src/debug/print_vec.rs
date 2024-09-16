use std::fmt::Debug;

use crate::prelude::*;

fn print_vec<T: Debug>(vec: &Vec<T>, categories: Vec<LogCategory>, level: BevyLogLevel) {
    if !LOG_LEVELS_TO_PRINT.contains(&level) {
        return;
    }
    let print_config = get_print_config(categories, level);

    for element in vec {
        debug_print_by_config(element, print_config);
    }
}

pub fn print_error_vec<T: Debug>(vec: &Vec<T>, log_categories: Vec<LogCategory>) {
    print_vec(vec, log_categories, BevyLogLevel::Error);
}

pub fn print_warning_vec<T: Debug>(vec: &Vec<T>, log_categories: Vec<LogCategory>) {
    print_vec(vec, log_categories, BevyLogLevel::Warning);
}

pub fn print_info_vec<T: Debug>(vec: &Vec<T>, log_categories: Vec<LogCategory>) {
    print_vec(vec, log_categories, BevyLogLevel::Info);
}
