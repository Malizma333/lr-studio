use std::collections::HashMap;

use crate::entity::{
    joint::EntityJoint,
    registry::{EntityBoneId, EntityBoneTemplateId},
};

pub(crate) struct EntityJointTemplate {
    bones_involved: (EntityBoneTemplateId, EntityBoneTemplateId),
}

impl EntityJointTemplate {
    pub fn new(
        bones_involved: (EntityBoneTemplateId, EntityBoneTemplateId),
    ) -> EntityJointTemplate {
        EntityJointTemplate { bones_involved }
    }

    pub fn build(&self, mapping: &HashMap<EntityBoneTemplateId, EntityBoneId>) -> EntityJoint {
        EntityJoint {
            bones_involved: (
                mapping[&self.bones_involved.0],
                mapping[&self.bones_involved.1],
            ),
        }
    }
}
