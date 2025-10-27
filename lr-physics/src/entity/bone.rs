use crate::entity::{
    bone::snapshot::EntityBoneSnapshot,
    registry::{EntityPointId, EntityRegistry},
};

pub(crate) mod snapshot;
pub(crate) mod template;

pub(crate) struct EntityBone {
    connected_points: (EntityPointId, EntityPointId),
    initial_length: f64,
    bias: f64,
    initial_length_factor: f64,
    repel_only: bool,
    endurance: f64,
    adjustment_strength: f64,
    endurance_remount_factor: f64,
    adjustment_strength_remount_factor: f64,
}

impl EntityBone {
    pub fn get_snapshot(&self, registry: &EntityRegistry, remounting: bool) -> EntityBoneSnapshot {
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
        let rest_length = self.initial_length * self.initial_length_factor;

        EntityBoneSnapshot::new(
            vector,
            adjustment_strength,
            self.bias,
            endurance,
            rest_length,
            self.repel_only,
            is_flutter,
        )
    }

    pub fn get_points(&self) -> (EntityPointId, EntityPointId) {
        self.connected_points
    }
}
