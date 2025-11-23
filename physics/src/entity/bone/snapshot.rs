use vector2d::Vector2Df;

pub(crate) struct EntityBoneSnapshot {
    pub(super) vector: Vector2Df,
    pub(super) rest_length: f64,
    pub(super) is_repel: bool,
    pub(super) is_flutter: bool,
    pub(super) adjustment_strength: f64,
    pub(super) endurance: f64,
    pub(super) bias: f64,
}

impl EntityBoneSnapshot {
    fn get_percent_adjustment(&self) -> f64 {
        let bone_vector = self.vector;
        let current_length = bone_vector.length();
        let should_repel = current_length < self.rest_length;

        if current_length == 0.0 || (self.is_repel && !should_repel) {
            0.0
        } else {
            (current_length - self.rest_length) / current_length
        }
    }

    fn get_adjustment(&self) -> (Vector2Df, Vector2Df) {
        let bone_vector = self.vector;
        let percent_adjustment = self.get_percent_adjustment();
        let adjustment_strength = self.adjustment_strength;
        (
            -1.0 * bone_vector * adjustment_strength * percent_adjustment * (1.0 - self.bias),
            -1.0 * bone_vector * adjustment_strength * percent_adjustment * self.bias,
        )
    }

    fn get_intact(&self) -> bool {
        let percent_adjustment = self.get_percent_adjustment();
        let endurance = self.endurance;
        percent_adjustment <= endurance * self.rest_length
    }
}

#[cfg(test)]
mod tests {
    use std::f64::INFINITY;

    use vector2d::Vector2Df;

    use crate::entity::bone::snapshot::EntityBoneSnapshot;

    #[test]
    fn get_percent_adjustment() {
        let bone = EntityBoneSnapshot {
            vector: Vector2Df::zero(),
            adjustment_strength: 1.0,
            bias: 1.0,
            endurance: 1.0,
            rest_length: 1.0,
            is_repel: false,
            is_flutter: false,
        };
        assert!(
            bone.get_percent_adjustment() == 0.0,
            "bone of length zero should have no adjustment"
        );
        let bone = EntityBoneSnapshot {
            vector: Vector2Df::up() * 2.0,
            adjustment_strength: 1.0,
            bias: 1.0,
            endurance: 1.0,
            rest_length: 1.0,
            is_repel: false,
            is_flutter: false,
        };
        assert!(
            bone.get_percent_adjustment() == 0.5,
            "bone adjustment should be correct"
        );
        let bone = EntityBoneSnapshot {
            vector: Vector2Df::up() * 0.5,
            adjustment_strength: 1.0,
            bias: 1.0,
            endurance: 1.0,
            rest_length: 1.0,
            is_repel: true,
            is_flutter: false,
        };
        assert!(
            bone.get_percent_adjustment() != 0.0,
            "repel bone should repel when under rest length"
        );
        let bone = EntityBoneSnapshot {
            vector: Vector2Df::up() * 2.0,
            adjustment_strength: 1.0,
            bias: 1.0,
            endurance: 1.0,
            rest_length: 1.0,
            is_repel: true,
            is_flutter: false,
        };
        assert!(
            bone.get_percent_adjustment() == 0.0,
            "repel bone should not repel when over rest length"
        );
        let bone = EntityBoneSnapshot {
            vector: Vector2Df::up(),
            adjustment_strength: 1.0,
            bias: 1.0,
            endurance: 1.0,
            rest_length: 5.0,
            is_repel: false,
            is_flutter: false,
        };
        assert!(
            bone.get_percent_adjustment() == -4.0,
            "large rest length factor should give correct result"
        );
        let bone = EntityBoneSnapshot {
            vector: Vector2Df::up(),
            adjustment_strength: 1.0,
            bias: 1.0,
            endurance: 1.0,
            rest_length: 0.25,
            is_repel: false,
            is_flutter: false,
        };
        assert!(
            bone.get_percent_adjustment() == 0.75,
            "small rest length factor should give correct result"
        );
    }

    #[test]
    fn get_adjustment() {
        let bone = EntityBoneSnapshot {
            vector: Vector2Df::up() * 2.0,
            adjustment_strength: 1.0,
            bias: 0.5,
            endurance: 1.0,
            rest_length: 1.0,
            is_repel: false,
            is_flutter: false,
        };
        assert!(
            bone.get_adjustment() == (-0.5 * Vector2Df::up(), -0.5 * Vector2Df::up()),
            "bone with half bias should adjust both equally"
        );
        let bone = EntityBoneSnapshot {
            vector: Vector2Df::up() * 2.0,
            adjustment_strength: 1.0,
            bias: 0.0,
            endurance: 1.0,
            rest_length: 1.0,
            is_repel: false,
            is_flutter: false,
        };
        assert!(
            bone.get_adjustment() == (-1.0 * Vector2Df::up(), 0.0 * Vector2Df::up()),
            "bone with zero bias should only adjust first value"
        );
        let bone = EntityBoneSnapshot {
            vector: Vector2Df::up() * 2.0,
            adjustment_strength: 1.0,
            bias: 1.0,
            endurance: 1.0,
            rest_length: 1.0,
            is_repel: false,
            is_flutter: false,
        };
        assert!(
            bone.get_adjustment() == (0.0 * Vector2Df::up(), -1.0 * Vector2Df::up()),
            "bone with one bias should only adjust second value"
        );
        let bone = EntityBoneSnapshot {
            vector: Vector2Df::zero(),
            adjustment_strength: 1.0,
            bias: 1.0,
            endurance: 1.0,
            rest_length: 1.0,
            is_repel: false,
            is_flutter: false,
        };
        assert!(
            bone.get_adjustment() == (Vector2Df::zero(), Vector2Df::zero()),
            "bone with current length 0 should have no adjustment"
        );
        let bone = EntityBoneSnapshot {
            vector: Vector2Df::up() * 2.0,
            adjustment_strength: 1.0,
            bias: 1.0,
            endurance: 1.0,
            rest_length: 1.0,
            is_repel: true,
            is_flutter: false,
        };
        assert!(
            bone.get_adjustment() == (Vector2Df::zero(), Vector2Df::zero()),
            "repel bone over rest length should not have adjustment"
        );
        let bone = EntityBoneSnapshot {
            vector: Vector2Df::up(),
            adjustment_strength: 1.0,
            bias: 1.0,
            endurance: 1.0,
            rest_length: 1.0,
            is_repel: false,
            is_flutter: false,
        };
        assert!(
            bone.get_adjustment() == (Vector2Df::zero(), Vector2Df::zero()),
            "repel bone over rest length should not have adjustment"
        );
        let bone = EntityBoneSnapshot {
            vector: Vector2Df::up(),
            adjustment_strength: 1.0,
            bias: 0.5,
            endurance: 1.0,
            rest_length: 5.0,
            is_repel: false,
            is_flutter: false,
        };
        assert!(
            bone.get_adjustment() == (2.0 * Vector2Df::up(), 2.0 * Vector2Df::up()),
            "large rest length factor should give correct result"
        );
        let bone = EntityBoneSnapshot {
            vector: Vector2Df::up(),
            adjustment_strength: 1.0,
            bias: 0.5,
            endurance: 1.0,
            rest_length: 0.25,
            is_repel: false,
            is_flutter: false,
        };
        assert!(
            bone.get_adjustment() == (-0.375 * Vector2Df::up(), -0.375 * Vector2Df::up()),
            "small rest length factor should give correct result"
        );
        let bone = EntityBoneSnapshot {
            vector: Vector2Df::up() * 2.0,
            adjustment_strength: 6.0,
            bias: 0.5,
            endurance: 1.0,
            rest_length: 1.0,
            is_repel: false,
            is_flutter: false,
        };
        assert!(
            bone.get_adjustment() == (-3.0 * Vector2Df::up(), -3.0 * Vector2Df::up()),
            "adjustment strength should scale adjustment"
        );
    }

    #[test]
    fn get_intact() {
        let bone = EntityBoneSnapshot {
            vector: Vector2Df::zero(),
            adjustment_strength: 1.0,
            bias: 0.5,
            endurance: 1.0,
            rest_length: 1.0,
            is_repel: false,
            is_flutter: false,
        };
        assert!(bone.get_intact(), "length 0 bone should be intact");
        let bone = EntityBoneSnapshot {
            vector: Vector2Df::one() * 2.0,
            adjustment_strength: 1.0,
            bias: 0.5,
            endurance: 1.0,
            rest_length: 1.0,
            is_repel: true,
            is_flutter: false,
        };
        assert!(
            bone.get_intact(),
            "repel bone over rest length should be intact"
        );
        let bone = EntityBoneSnapshot {
            vector: Vector2Df::one() * 5.0,
            adjustment_strength: 1.0,
            bias: 0.5,
            endurance: INFINITY,
            rest_length: 1.0,
            is_repel: false,
            is_flutter: false,
        };
        assert!(bone.get_intact(), "infinite endurance should be intact");
        let bone = EntityBoneSnapshot {
            vector: Vector2Df::one() * 5.0,
            adjustment_strength: 1.0,
            bias: 0.5,
            endurance: 0.25,
            rest_length: 1.0,
            is_repel: false,
            is_flutter: false,
        };
        assert!(!bone.get_intact(), "small endurance should not be intact");
    }
}
