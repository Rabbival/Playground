use crate::prelude::*;

#[derive(Debug, Event, Clone, Copy)]
pub struct AddTimerToEntity<T: Numeric> {
    pub timer: CustomTimer<T>,
    pub attach_to: Entity,
}

pub struct AddTimerToEntityPlugin;

impl Plugin for AddTimerToEntityPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AddTimerToEntity<f32>>()
            .add_event::<AddTimerToEntity<Vec2>>()
            .add_event::<AddTimerToEntity<Vec3>>()
            .add_event::<AddTimerToEntity<Quat>>();
    }
}
