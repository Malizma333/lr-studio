use vector2d::Vector2Df;

use crate::entity::registry::EntityBoneId;

pub(crate) struct EntityJoint {
    pub(super) bones: (EntityBoneId, EntityBoneId),
    pub(super) mount: bool,
}

impl EntityJoint {
    pub(crate) fn should_break(&self, bone_vectors: (Vector2Df, Vector2Df)) -> bool {
        Vector2Df::cross(bone_vectors.0, bone_vectors.1) < 0.0
    }

    pub(crate) fn is_mount(&self) -> bool {
        self.mount
    }

    pub(crate) fn bones(&self) -> (EntityBoneId, EntityBoneId) {
        self.bones
    }
}
