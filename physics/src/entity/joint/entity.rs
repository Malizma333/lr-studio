use vector2d::Vector2Df;

use crate::entity::{
    joint::snapshot::EntityJointSnapshot,
    registry::{EntityBoneId, EntityRegistry},
};

pub(crate) struct EntityJoint {
    pub(super) bones_involved: (EntityBoneId, EntityBoneId),
    pub(super) mount: bool,
}

impl EntityJoint {
    pub(crate) fn get_snapshot(&self, registry: &EntityRegistry) -> EntityJointSnapshot {
        // Don't care about remounting when getting joint snapshot
        let remounting = false;
        let bones = (
            registry
                .get_bone(self.bones_involved.0)
                .get_snapshot(registry, remounting),
            registry
                .get_bone(self.bones_involved.0)
                .get_snapshot(registry, remounting),
        );
        // TODO
        EntityJointSnapshot {
            bone_vectors: (Vector2Df::zero(), Vector2Df::zero()),
        }
    }
}
