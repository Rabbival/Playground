use crate::prelude::*;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum InputSystemSet {
    ListeningPreperations,
    Listening,
    Handling,
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum AnimationSystemSet {
    PreTickingPreperations,
    PreTicking,
    PostTicking,
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum TimerSystemSet {
    TimerTicking,
    TimerAttachment,
    TimeMultipliersUpdating,
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
                    AnimationSystemSet::PreTickingPreperations,
                    AnimationSystemSet::PreTicking,
                    TimerSystemSet::TimerTicking,
                )
                    .chain()
                    .after(InputSystemSet::Handling),
                (
                    TimerSystemSet::TimeMultipliersUpdating,
                    TimerSystemSet::TimerAttachment,
                    AnimationSystemSet::PostTicking,
                )
                    .after(TimerSystemSet::TimerTicking),
            ),
        );
    }
}
