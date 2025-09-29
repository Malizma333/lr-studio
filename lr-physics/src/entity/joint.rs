use vector2d::Vector2Df;

use crate::engine::{Engine, EntityRegistryIndex};

pub struct EntityJoint {
    bones_involved: (EntityRegistryIndex, EntityRegistryIndex),
}

pub struct EntityJointBuilder {
    bones_involved: Option<(EntityRegistryIndex, EntityRegistryIndex)>,
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

    pub fn bones(&mut self, bone1: EntityRegistryIndex, bone2: EntityRegistryIndex) -> &mut Self {
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
    pub fn get_intact(&self, engine: &Engine) -> bool {
        Vector2Df::cross(
            engine.get_bone(self.bones_involved.0).get_vector(engine),
            engine.get_bone(self.bones_involved.1).get_vector(engine),
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
