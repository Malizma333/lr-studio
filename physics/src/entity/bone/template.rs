use std::collections::HashMap;

use vector2d::Vector2Df;

use crate::entity::{
    bone::entity::EntityBone,
    registry::{EntityPointId, EntityPointTemplateId},
};

pub(crate) struct EntityBoneTemplate {
    pub(super) connected_points: (EntityPointTemplateId, EntityPointTemplateId),
    pub(super) bias: f64,
    pub(super) initial_length_factor: f64,
    pub(super) repel_only: bool,
    pub(super) endurance: f64,
    pub(super) adjustment_strength: f64,
    pub(super) endurance_remount_factor: f64,
    pub(super) adjustment_strength_remount_factor: f64,
}

impl EntityBoneTemplate {
    pub fn build(
        &self,
        point_mapping: &HashMap<EntityPointTemplateId, EntityPointId>,
    ) -> EntityBone {
        let connected_points = (
            point_mapping[&self.connected_points.0],
            point_mapping[&self.connected_points.1],
        );
        // TODO
        let rest_length =
            Vector2Df::distance(Vector2Df::zero(), Vector2Df::zero()) * self.initial_length_factor;

        EntityBone {
            connected_points,
            rest_length,
            bias: self.bias,
            is_repel: self.repel_only,
            endurance: self.endurance,
            adjustment_strength: self.adjustment_strength,
            endurance_remount_factor: self.endurance_remount_factor,
            adjustment_strength_remount_factor: self.endurance_remount_factor,
        }
    }
}
