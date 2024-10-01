use crate::prelude::*;

#[derive(Debug, Event, Clone, Copy)]
pub struct DestroyValueCalculator(pub Entity);

pub struct DestroyValueCalculatorPlugin;

impl Plugin for DestroyValueCalculatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DestroyValueCalculator>();
    }
}
