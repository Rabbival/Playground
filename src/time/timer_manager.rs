use crate::{prelude::*, read_single_field_variant};

pub struct TimerManagerPlugin;

impl Plugin for TimerManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                (
                    tick_timers::<f32>,
                    tick_timers::<Vec2>,
                    tick_timers::<Vec3>,
                    tick_timers::<Quat>,
                ),
                (
                    listen_for_time_processors_requests::<f32>,
                    listen_for_time_processors_requests::<Vec2>,
                    listen_for_time_processors_requests::<Vec3>,
                    listen_for_time_processors_requests::<Quat>,
                ),
            )
                .chain(),
        );
    }
}

fn tick_timers<T: Numeric>(
    mut timer_event_writer: EventWriter<TimerEventChannel<T>>,
    mut timers: Query<(&mut CustomTimer<T>, Entity)>,
    time_processors: Res<TimeProcessors>,
    time: Res<Time>,
    mut commands: Commands,
) {
    let time_delta = time.delta_seconds();
    for (mut timer, timer_entity) in &mut timers {
        let time_multiplier = get_time_multiplier(&time_processors, &timer);
        tick_and_send_timer_event(
            time_delta * time_multiplier,
            &mut timer,
            timer_entity,
            &mut timer_event_writer,
            &mut commands,
        );
    }
}

fn get_time_multiplier<T: Numeric>(
    time_processors: &Res<TimeProcessors>,
    timer: &CustomTimer<T>,
) -> f32 {
    for (processor_id, time_processor) in time_processors.iter() {
        if timer.time_processor == *processor_id {
            return time_processor.time_multiplier();
        }
    }
    print_warning(
        GenericTimeRelatedError::NoTimeProcessorAssignedToTimer(*timer),
        vec![LogCategory::RequestNotFulfilled],
    );
    1.0
}

fn tick_and_send_timer_event<T: Numeric>(
    time_to_tick: f32,
    timer: &mut CustomTimer<T>,
    timer_entity: Entity,
    timer_event_writer: &mut EventWriter<TimerEventChannel<T>>,
    commands: &mut Commands,
) {
    if let Some(timer_event) = timer.tick_and_get_event(time_to_tick) {
        timer_event_writer.send(TimerEventChannel::EventFromTimer(timer_event));
    }
    if timer.finished() {
        commands.entity(timer_entity).despawn();
    }
}

// TODO: Assign this and previous functions to chained system sets then move it to time processors
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
            } => {
                let maybe_time_processor = time_processors.get_mut(*processor_id);
                if let Some(time_processor) = maybe_time_processor {
                    if time_processor.changeable_time_multiplier() {
                        commands.spawn(CustomTimer::<f32>::new(
                            TimeProcessorId::default(),
                            *duration,
                            time_processor.time_multiplier(),
                            *new_multiplier,
                            Some(EventFromTimerType::ChangeTimeProcessorSpeed(*processor_id)),
                            None,
                        ));
                    } else {
                        print_warning(
                            NonGenericTimeRelatedError::AttemptedToChangeFixedMultiplierTimeProcessor(
                                *processor_id,
                            ),
                            vec![LogCategory::RequestNotFulfilled],
                        );
                    }
                } else {
                    print_warning(
                        NonGenericTimeRelatedError::TimeProcessorNotFound(*processor_id),
                        vec![LogCategory::RequestNotFulfilled],
                    );
                }
            }
            TimeProcessorsRequest::AddProcessor(time_processor) => {
                time_processors.add(*time_processor);
            }
        }
    }
}
