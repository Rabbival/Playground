use crate::{prelude::*, read_single_field_variant};

pub struct TimeProcessorsUpdatePlugin;

impl Plugin for TimeProcessorsUpdatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                listen_for_time_processors_requests::<f32>,
                listen_for_time_processors_requests::<Vec2>,
                listen_for_time_processors_requests::<Vec3>,
                listen_for_time_processors_requests::<Quat>,
            )
                .in_set(TimerSystemSet::TimeProcessorsUpdating),
        );
    }
}

fn listen_for_time_processors_requests<T: Numeric>(
    mut timer_event_reader: EventReader<TimerEventChannel<T>>,
    mut time_processors: ResMut<TimeProcessors>,
    mut commands: Commands,
) {
    for time_processors_request in
        read_single_field_variant!(timer_event_reader, TimerEventChannel::ProcessorsRequest)
    {
        match time_processors_request {
            TimeProcessorsRequest::SetTimeMultiplier {
                processor_id,
                new_multiplier,
                duration,
            } => set_time_multiplier(
                &mut time_processors,
                &mut commands,
                *processor_id,
                *new_multiplier,
                *duration,
            ),
            TimeProcessorsRequest::AddProcessor(time_processor) => {
                time_processors.add(*time_processor);
            }
        }
    }
}

fn set_time_multiplier(
    time_processors: &mut ResMut<TimeProcessors>,
    commands: &mut Commands,
    processor_id: TimeProcessorId,
    new_multiplier: f32,
    duration: f32,
) {
    let maybe_time_processor = time_processors.get_mut(processor_id);
    if let Some(time_processor) = maybe_time_processor {
        if time_processor.changeable_time_multiplier() {
            commands.spawn(CustomTimer::<f32>::new(
                TimeProcessorId::default(),
                duration,
                time_processor.time_multiplier(),
                new_multiplier,
                Some(EventFromTimerType::ChangeTimeProcessorSpeed(processor_id)),
                None,
            ));
        } else {
            print_warning(
                NonGenericTimeRelatedError::AttemptedToChangeFixedMultiplierTimeProcessor(
                    processor_id,
                ),
                vec![LogCategory::RequestNotFulfilled],
            );
        }
    } else {
        print_warning(
            NonGenericTimeRelatedError::TimeProcessorNotFound(processor_id),
            vec![LogCategory::RequestNotFulfilled],
        );
    }
}
