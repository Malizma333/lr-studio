use std::collections::HashMap;

use crate::entity::{
    joint::entity::EntityJoint,
    registry::{EntityBoneId, EntityBoneTemplateId},
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
