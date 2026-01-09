use std::collections::HashMap;

use crate::engine::entity_registry::{
    EntityPointId, EntityPointTemplateId, EntityRegistry, bone::entity::EntityBone,
};

pub(crate) struct EntityBoneTemplate {
    pub(super) connected_points: (EntityPointTemplateId, EntityPointTemplateId),
    pub(super) bias: f64,
    pub(super) initial_length_factor: f64,
    pub(super) repel_only: bool,
    pub(super) is_flutter: bool,
    pub(super) endurance: f64,
    pub(super) adjustment_strength: f64,
    pub(super) endurance_remount_factor: f64,
    pub(super) adjustment_strength_remount_factor: f64,
}

impl EntityBoneTemplate {
    pub fn build(
        &self,
        point_mapping: &HashMap<EntityPointTemplateId, EntityPointId>,
        registry: &EntityRegistry,
    ) -> EntityBone {
        let connected_points = (
            point_mapping[&self.connected_points.0],
            point_mapping[&self.connected_points.1],
        );
        let initial_positions = (
            registry.get_point(connected_points.0).initial_position(),
            registry.get_point(connected_points.1).initial_position(),
        );
        let rest_length =
            initial_positions.0.distance_from(initial_positions.1) * self.initial_length_factor;

        EntityBone {
            points: connected_points,
            rest_length,
            bias: self.bias,
            repel_only: self.repel_only,
            is_flutter: self.is_flutter,
            endurance: self.endurance,
            endurance_remount_factor: self.endurance_remount_factor,
            adjustment_strength: self.adjustment_strength,
            adjustment_strength_remount_factor: self.adjustment_strength_remount_factor,
        }
    }
}
