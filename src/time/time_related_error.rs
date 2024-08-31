use std::fmt::{Debug, Display};

use crate::prelude::*;

#[derive(Debug, Copy, Clone)]
pub enum TimeRelatedError {
    TimeMultiplierNotFound(TimeMultiplierId),
    AttemptedToChangeFixedTimeMultiplier(TimeMultiplierId),
    TimerToRemoveFromNotFound(RemoveFromTimerAffectedEntities),
    FullTimerAffectedEntitiesError(
        VecBasedArrayError<FullTimerAffectedEntity, TIMER_MAX_ASSIGNED_ENTITIES>,
    ),
    OnceDoneTimerAffectedEntitiesError(VecBasedArrayError<Entity, TIMER_MAX_ASSIGNED_ENTITIES>),
}

impl Display for TimeRelatedError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::TimeMultiplierNotFound(id) => {
                write!(f, "Time processor with id {:?} not found", id)
            }
            Self::AttemptedToChangeFixedTimeMultiplier(id) => write!(
                f,
                "Attempted to change fixed multiplier time processor with id {:?}",
                id
            ),
            Self::TimerToRemoveFromNotFound(event) => {
                write!(
                    f,
                    "Couldn't find timer to remove entity from. Event: {:?}",
                    event
                )
            }
            Self::FullTimerAffectedEntitiesError(vec_based_array_error) => {
                write!(
                    f,
                    "Error when accessing full timer affected entities: {:?}",
                    vec_based_array_error
                )
            }
            Self::OnceDoneTimerAffectedEntitiesError(vec_based_array_error) => {
                write!(
                    f,
                    "Error when accessing once done timer affected entities: {:?}",
                    vec_based_array_error
                )
            }
        }
    }
}

impl From<VecBasedArrayError<FullTimerAffectedEntity, TIMER_MAX_ASSIGNED_ENTITIES>>
    for TimeRelatedError
{
    fn from(
        value: VecBasedArrayError<FullTimerAffectedEntity, TIMER_MAX_ASSIGNED_ENTITIES>,
    ) -> Self {
        TimeRelatedError::FullTimerAffectedEntitiesError(value)
    }
}

impl From<VecBasedArrayError<Entity, TIMER_MAX_ASSIGNED_ENTITIES>> for TimeRelatedError {
    fn from(value: VecBasedArrayError<Entity, TIMER_MAX_ASSIGNED_ENTITIES>) -> Self {
        TimeRelatedError::OnceDoneTimerAffectedEntitiesError(value)
    }
}
