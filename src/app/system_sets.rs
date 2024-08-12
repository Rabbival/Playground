use crate::prelude::*;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum InputSystemSet {
    ListeningPreperations,
    Listening,
    Handling,
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum TimerSystemSet {
    PreTickingPreperations,
    PreTicking,
    TimerTicking,
    PostTicking,
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum EndOfFrameSystemSet {
    TimerClearing,
    LateDespawn,
}

pub struct SystemSetsPlugin;

impl Plugin for SystemSetsPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (
                (
                    InputSystemSet::ListeningPreperations,
                    InputSystemSet::Listening,
                    InputSystemSet::Handling,
                )
                    .chain(),
                (
                    TimerSystemSet::PreTickingPreperations,
                    TimerSystemSet::PreTicking,
                    TimerSystemSet::TimerTicking,
                    TimerSystemSet::PostTicking,
                )
                    .chain()
                    .after(InputSystemSet::Handling),
                (
                    EndOfFrameSystemSet::TimerClearing,
                    EndOfFrameSystemSet::LateDespawn,
                )
                    .after(TimerSystemSet::PostTicking),
            ),
        );
    }
}
