use crate::prelude::*;
use std::fmt::Display;

fn print_log<T: Display>(message: T, categories: Vec<LogCategory>, level: BevyLogLevel) {
    if !LOG_LEVELS_TO_PRINT.contains(&level) {
        return;
    }
    let log_message = format!("{}", message);
    let mut print_message = false;
    let mut append_message_to_session_log = false;
    for category in categories {
        if LOG_CATEGORYS_TO_PRINT.contains(&category) {
            print_message = true;
        }
        if LOG_CATEGORYS_TO_APPEND_TO_SESSION_LOG.contains(&category) {
            append_message_to_session_log = true;
        }
        if print_message && append_message_to_session_log {
            break;
        }
    }

    if print_message {
        match level {
            BevyLogLevel::Error => error!(log_message),
            BevyLogLevel::Warning => warn!(log_message),
            BevyLogLevel::Info => info!(log_message),
        }
    }
    if append_message_to_session_log {
        append_to_game_session_log_file(log_message);
    }
}

pub fn print_error<T: Display>(message: T, log_categories: Vec<LogCategory>) {
    print_log(message, log_categories, BevyLogLevel::Error);
}

pub fn print_warning<T: Display>(message: T, log_categories: Vec<LogCategory>) {
    print_log(message, log_categories, BevyLogLevel::Warning);
}

pub fn print_info<T: Display>(message: T, log_categories: Vec<LogCategory>) {
    print_log(message, log_categories, BevyLogLevel::Info);
}
