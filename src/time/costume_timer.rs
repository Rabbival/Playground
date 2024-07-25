use crate::prelude::*;

#[derive(Debug, Component)]
pub struct CostumeTimer {
    pub timer: Timer,
    pub send_as_going: Option<TimerEvent>,
    pub send_once_done: Option<TimerEvent>,
}
