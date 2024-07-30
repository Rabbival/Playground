use crate::prelude::*;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum InputSystemSet {
    ListeningPreperations,
    Listening,
    Handling,
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum AnimationSystemSet {
    ListeningPreperations,
    Listening,
    Handling,
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum TimerSystemSet {
    TimerTicking,
    TimeProcessorsUpdating,
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
                    TimerSystemSet::TimerTicking,
                    TimerSystemSet::TimeProcessorsUpdating,
                    AnimationSystemSet::ListeningPreperations,
                    AnimationSystemSet::Listening,
                    AnimationSystemSet::Handling,
                )
                    .chain()
                    .after(InputSystemSet::Handling),
            ),
        );
    }
}
