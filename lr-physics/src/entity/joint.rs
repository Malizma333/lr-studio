use vector2d::Vector2Df;

use crate::{
    engine::{
        entity_registry::{EntityRegistry, EntityRegistryIndex},
        timeline_engine::Engine,
    },
    entity::bone::EntityBoneLogic,
};

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

pub trait EntityJointLogic {
    fn bone_vectors(&self) -> (Vector2Df, Vector2Df);

    fn is_intact(&self) -> bool {
        let bone_vectors = self.bone_vectors();
        Vector2Df::cross(bone_vectors.0, bone_vectors.1) < 0.0
    }
}

pub struct EntityJointSnapshot {
    bone_vectors: (Vector2Df, Vector2Df),
}

impl EntityJointLogic for EntityJointSnapshot {
    fn bone_vectors(&self) -> (Vector2Df, Vector2Df) {
        self.bone_vectors
    }
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

#[cfg(test)]
mod tests {
    use vector2d::Vector2Df;

    use crate::entity::joint::EntityJointLogic;

    struct PureJoint(pub Vector2Df, pub Vector2Df);

    impl EntityJointLogic for PureJoint {
        fn bone_vectors(&self) -> (Vector2Df, Vector2Df) {
            (self.0, self.1)
        }
    }

    #[test]
    fn get_intact() {
        let joint = PureJoint(Vector2Df::new(0.0, 5.0), Vector2Df::new(0.0, 3.0));
        assert!(!joint.is_intact(), "parallel bones should not be intact");
        let joint = PureJoint(Vector2Df::new(0.0, 5.0), Vector2Df::new(0.0, -3.0));
        assert!(
            !joint.is_intact(),
            "anti-parallel bones should not be intact"
        );
        let joint = PureJoint(Vector2Df::new(0.0, 5.0), Vector2Df::new(-3.0, 0.0));
        assert!(
            !joint.is_intact(),
            "perpendicular bones (counterclockwise order) should not be intact"
        );
        let joint = PureJoint(Vector2Df::new(-3.0, 0.0), Vector2Df::new(0.0, 5.0));
        assert!(
            joint.is_intact(),
            "perpendicular bones (clockwise order) should be intact"
        );
        let joint = PureJoint(Vector2Df::new(4.0, 7.0), Vector2Df::new(-1.0, 6.0));
        assert!(
            !joint.is_intact(),
            "positive angle between bones should not be intact"
        );
        let joint = PureJoint(Vector2Df::new(5.0, 3.0), Vector2Df::new(7.0, -3.0));
        assert!(
            joint.is_intact(),
            "negative angle between bones should be intact"
        );
    }
}
