use strum::IntoEnumIterator;

use crate::prelude::*;

#[derive(Debug, Resource, Default)]
pub struct TimeMultipliers(HashMap<TimeMultiplierId, TimeMultiplier>);

pub struct TimeMultipliersInitPlugin;

impl Plugin for TimeMultipliersInitPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TimeMultipliers>()
            .add_systems(PreStartup, initialize_time_processors);
    }
}

fn initialize_time_processors(mut time_processors: ResMut<TimeMultipliers>) {
    *time_processors = TimeMultipliers(HashMap::from([(
        TimeMultiplierId::default(),
        TimeMultiplier::new(TimeMultiplierId::default(), 1.0, false),
    )]));
    for time_processor_id in TimeMultiplierId::iter() {
        time_processors.add(time_processor_id.to_initial_properties());
    }
}

impl TimeMultipliers {
    pub fn add(&mut self, processor: TimeMultiplier) {
        if processor.id() != TimeMultiplierId::default() {
            self.0.insert(processor.id(), processor);
        } else {
            print_warning(
                "Tried to set value for default time processor",
                vec![LogCategory::RequestNotFulfilled],
            );
        }
    }

    pub fn get(&self, id: TimeMultiplierId) -> Option<&TimeMultiplier> {
        self.0.get(&id)
    }

    pub fn get_mut(&mut self, id: TimeMultiplierId) -> Option<&mut TimeMultiplier> {
        self.0.get_mut(&id)
    }

    pub fn remove(&mut self, id: TimeMultiplierId) -> Option<TimeMultiplier> {
        self.0.remove(&id)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&TimeMultiplierId, &TimeMultiplier)> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&TimeMultiplierId, &mut TimeMultiplier)> {
        self.0.iter_mut()
    }
}
