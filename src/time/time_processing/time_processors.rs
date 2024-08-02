use strum::IntoEnumIterator;

use crate::prelude::*;

#[derive(Debug, Resource, Default)]
pub struct TimeProcessors(HashMap<TimeProcessorId, TimeProcessor>);

pub struct TimeProcessorsInitPlugin;

impl Plugin for TimeProcessorsInitPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TimeProcessors>()
            .add_systems(PreStartup, initialize_time_processors);
    }
}

fn initialize_time_processors(mut time_processors: ResMut<TimeProcessors>) {
    *time_processors = TimeProcessors(HashMap::from([(
        TimeProcessorId::default(),
        TimeProcessor::new(TimeProcessorId::default(), 1.0, false),
    )]));
    for time_processor_id in TimeProcessorId::iter() {
        time_processors.add(time_processor_id.to_initial_properties());
    }
}

impl TimeProcessors {
    pub fn add(&mut self, processor: TimeProcessor) {
        if processor.id() != TimeProcessorId::default() {
            self.0.insert(processor.id(), processor);
        } else {
            print_warning(
                "Tried to set value for default time processor",
                vec![LogCategory::RequestNotFulfilled],
            );
        }
    }

    pub fn get(&self, id: TimeProcessorId) -> Option<&TimeProcessor> {
        self.0.get(&id)
    }

    pub fn get_mut(&mut self, id: TimeProcessorId) -> Option<&mut TimeProcessor> {
        self.0.get_mut(&id)
    }

    pub fn remove(&mut self, id: TimeProcessorId) -> Option<TimeProcessor> {
        self.0.remove(&id)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&TimeProcessorId, &TimeProcessor)> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&TimeProcessorId, &mut TimeProcessor)> {
        self.0.iter_mut()
    }
}
