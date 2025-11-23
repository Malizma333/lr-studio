use crate::entity::{
    bone::snapshot::EntityBoneSnapshot,
    registry::{EntityPointId, EntityRegistry},
};

pub(crate) struct EntityBone {
    pub(super) connected_points: (EntityPointId, EntityPointId),
    pub(super) rest_length: f64,
    pub(super) bias: f64,
    pub(super) is_repel: bool,
    pub(super) endurance: f64,
    pub(super) adjustment_strength: f64,
    pub(super) endurance_remount_factor: f64,
    pub(super) adjustment_strength_remount_factor: f64,
}

impl EntityBone {
    pub(crate) fn get_snapshot(
        &self,
        registry: &EntityRegistry,
        remounting: bool,
    ) -> EntityBoneSnapshot {
        let p0 = registry.get_point(self.connected_points.0).get_snapshot();
        let p1 = registry.get_point(self.connected_points.1).get_snapshot();
        let is_flutter = !(p0.is_contact() && p1.is_contact());
        let vector = p0.position() - p1.position();
        let adjustment_strength = if remounting {
            self.adjustment_strength * self.adjustment_strength_remount_factor
        } else {
            self.adjustment_strength
        };
        let endurance = if remounting {
            self.endurance * self.endurance_remount_factor
        } else {
            self.endurance
        };

        EntityBoneSnapshot {
            vector,
            rest_length: self.rest_length,
            adjustment_strength,
            bias: self.bias,
            endurance,
            is_repel: self.is_repel,
            is_flutter,
        }
    }
}
