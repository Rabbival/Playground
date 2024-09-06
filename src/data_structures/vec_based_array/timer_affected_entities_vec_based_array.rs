use crate::prelude::*;

impl<const N: usize> VecBasedArray<TimerAffectedEntity, N> {
    pub fn affected_entities_iter(&self) -> impl Iterator<Item = Entity> + '_ {
        self.iter()
            .map(|affected_entity| affected_entity.affected_entity)
    }

    pub fn calculator_entities_iter(&self) -> impl Iterator<Item = Entity> + '_ {
        self.iter()
            .filter_map(|affected_entity| affected_entity.value_calculator_entity)
    }

    pub fn remove_by_affected_entity(
        &mut self,
        entity_to_remove: Entity,
    ) -> Result<TimerAffectedEntity, VecBasedArrayError<TimerAffectedEntity, Entity, N>> {
        self.remove_first_matching_item(entity_to_remove, |entity_to_remove, affected_entity| {
            entity_to_remove == affected_entity.affected_entity
        })
    }

    pub fn remove_by_calculator_entity(
        &mut self,
        calculator_to_remove: Entity,
    ) -> Result<TimerAffectedEntity, VecBasedArrayError<TimerAffectedEntity, Entity, N>> {
        self.remove_first_matching_item(calculator_to_remove, Self::calculator_matcher)
    }

    pub fn get_by_calculator_entity(
        &self,
        calculator_to_remove: Entity,
    ) -> Option<TimerAffectedEntity> {
        self.get_first_matching_item(calculator_to_remove, Self::calculator_matcher)
            .map(|(_, item)| item)
    }

    fn calculator_matcher(
        calculator_to_remove: Entity,
        affected_entity: TimerAffectedEntity,
    ) -> bool {
        matches!(affected_entity.value_calculator_entity, Some(calculator_entity) if calculator_entity == calculator_to_remove)
    }
}
