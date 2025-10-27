use crate::entity::{
    joint::snapshot::EntityJointSnapshot,
    registry::{EntityBoneId, EntityRegistry},
};

pub(crate) mod snapshot;
pub(crate) mod template;

pub struct EntityJoint {
    bones_involved: (EntityBoneId, EntityBoneId),
}

impl EntityJoint {
    pub fn get_snapshot(&self, registry: &EntityRegistry) -> EntityJointSnapshot {
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
        EntityJointSnapshot {
            bone_vectors: (bones.0.vector(), bones.1.vector()),
        }
    }
}
