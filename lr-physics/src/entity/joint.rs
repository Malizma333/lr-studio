use vector2d::Vector2Df;

use crate::entity::bone::{EntityBone, EntityBoneIndex};

fn get_bone(index: &EntityBoneIndex) -> &EntityBone {
    todo!("Need to get this from a registry")
}

pub type EntityJointIndex = usize;

pub struct EntityJoint {
    bones_involved: (EntityBoneIndex, EntityBoneIndex),
}

pub struct EntityJointBuilder {
    bones_involved: Option<(EntityBoneIndex, EntityBoneIndex)>,
}

#[derive(Debug, Clone)]
pub enum EntityJointBuilderError {
    MissingBones,
}

impl EntityJointBuilder {
    pub fn new() -> EntityJointBuilder {
        EntityJointBuilder {
            bones_involved: None,
        }
    }

    pub fn bones(&mut self, bone1: EntityBoneIndex, bone2: EntityBoneIndex) -> &mut Self {
        self.bones_involved = Some((bone1, bone2));
        self
    }

    pub fn build(&self) -> Result<EntityJoint, EntityJointBuilderError> {
        if let Some(bones_involved) = self.bones_involved {
            Ok(EntityJoint { bones_involved })
        } else {
            Err(EntityJointBuilderError::MissingBones)
        }
    }
}

impl EntityJoint {
    pub fn get_intact(&self) -> bool {
        Vector2Df::cross(
            get_bone(&self.bones_involved.0).get_vector(),
            get_bone(&self.bones_involved.1).get_vector(),
        ) < 0.0
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn get_intact() {
        todo!("bones parallel");
        todo!("bones perpendicular");
        todo!("bones positive angled not crossing");
        todo!("bones negative angled not crossing");
    }
}
