use std::collections::HashMap;

use vector2d::Vector2Df;

use crate::entity::{
    bone::entity::EntityBone,
    registry::{EntityPointId, EntityPointTemplateId, EntityRegistry},
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
        let rest_length = Vector2Df::distance(initial_positions.0, initial_positions.1)
            * self.initial_length_factor;

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
