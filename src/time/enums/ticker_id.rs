use crate::prelude::*;

#[derive(Debug, Component, Clone, Copy)]
pub enum TickerId {
    GameTimeTicker,
    UiTimeTicker,
}
