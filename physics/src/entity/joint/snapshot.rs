use vector2d::Vector2Df;

pub(crate) struct EntityJointSnapshot {
    pub(super) bone_vectors: (Vector2Df, Vector2Df),
}

impl EntityJointSnapshot {
    fn bone_vectors(&self) -> (Vector2Df, Vector2Df) {
        self.bone_vectors
    }

    fn should_break(&self) -> bool {
        let bone_vectors = self.bone_vectors();
        Vector2Df::cross(bone_vectors.0, bone_vectors.1) < 0.0
    }
}

#[cfg(test)]
mod tests {
    use vector2d::Vector2Df;

    use crate::entity::joint::snapshot::EntityJointSnapshot;

    #[test]
    fn get_intact() {
        let joint = EntityJointSnapshot {
            bone_vectors: (Vector2Df::new(0.0, 5.0), Vector2Df::new(0.0, 3.0)),
        };
        assert!(!joint.should_break(), "parallel bones should be intact");
        let joint = EntityJointSnapshot {
            bone_vectors: (Vector2Df::new(0.0, 5.0), Vector2Df::new(0.0, -3.0)),
        };
        assert!(
            !joint.should_break(),
            "anti-parallel bones should be intact"
        );
        let joint = EntityJointSnapshot {
            bone_vectors: (Vector2Df::new(0.0, 5.0), Vector2Df::new(-3.0, 0.0)),
        };
        assert!(
            !joint.should_break(),
            "perpendicular bones (counterclockwise order) should be intact"
        );
        let joint = EntityJointSnapshot {
            bone_vectors: (Vector2Df::new(-3.0, 0.0), Vector2Df::new(0.0, 5.0)),
        };
        assert!(
            joint.should_break(),
            "perpendicular bones (clockwise order) should not be intact"
        );
        let joint = EntityJointSnapshot {
            bone_vectors: (Vector2Df::new(4.0, 7.0), Vector2Df::new(-1.0, 6.0)),
        };
        assert!(
            !joint.should_break(),
            "positive angle between bones should be intact"
        );
        let joint = EntityJointSnapshot {
            bone_vectors: (Vector2Df::new(5.0, 3.0), Vector2Df::new(7.0, -3.0)),
        };
        assert!(
            joint.should_break(),
            "negative angle between bones should not be intact"
        );
    }
}
