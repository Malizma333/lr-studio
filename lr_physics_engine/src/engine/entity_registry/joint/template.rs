use std::collections::HashMap;

use crate::engine::entity_registry::{
    EntityBoneId, EntityBoneTemplateId, joint::entity::EntityJoint,
};

pub(crate) struct EntityJointTemplate {
    pub(super) bones_involved: (EntityBoneTemplateId, EntityBoneTemplateId),
    pub(super) mount: bool,
}

impl EntityJointTemplate {
    pub fn build(&self, bone_mapping: &HashMap<EntityBoneTemplateId, EntityBoneId>) -> EntityJoint {
        EntityJoint {
            bones: (
                bone_mapping[&self.bones_involved.0],
                bone_mapping[&self.bones_involved.1],
            ),
            mount: self.mount,
        }
    }
}
