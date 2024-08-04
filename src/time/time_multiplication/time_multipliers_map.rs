use strum::IntoEnumIterator;

use crate::prelude::*;

#[derive(Debug, Resource, Default)]
pub struct TimeMultipliersMap(HashMap<TimeMultiplierId, f32>);

impl TimeMultipliersMap {
    pub fn insert(&mut self, id: TimeMultiplierId, value: f32) {
        self.0.insert(id, value);
    }

    pub fn get(&self, id: &TimeMultiplierId) -> Option<&f32> {
        self.0.get(id)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&TimeMultiplierId, &f32)> {
        self.0.iter()
    }
}

pub struct TimeMultipliersMapPlugin;

impl Plugin for TimeMultipliersMapPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TimeMultipliersMap>()
            .add_systems(PreStartup, initialize_time_multipliers_map)
            .add_systems(Update, listen_for_multiplier_changes);
    }
}

fn initialize_time_multipliers_map(mut time_multipliers_map: ResMut<TimeMultipliersMap>) {
    for time_multiplier_id in TimeMultiplierId::iter() {
        let initial_properties = time_multiplier_id.to_initial_properties();
        time_multipliers_map
            .0
            .insert(initial_properties.id(), initial_properties.value());
    }
}

fn listen_for_multiplier_changes(
    time_multipliers: Query<&TimeMultiplier, Changed<TimeMultiplier>>,
    mut time_multipliers_map: ResMut<TimeMultipliersMap>,
) {
    for time_multiplier in &time_multipliers {
        time_multipliers_map
            .0
            .insert(time_multiplier.id(), time_multiplier.value());
    }
}
