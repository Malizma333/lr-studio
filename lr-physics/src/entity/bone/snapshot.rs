use vector2d::Vector2Df;

pub(crate) struct EntityBoneSnapshot {
    vector: Vector2Df,
    rest_length: f64,
    is_repel: bool,
    is_flutter: bool,
    adjustment_strength: f64,
    endurance: f64,
    bias: f64,
}

impl EntityBoneSnapshot {
    pub(super) fn new(
        vector: Vector2Df,
        adjustment_strength: f64,
        bias: f64,
        endurance: f64,
        rest_length: f64,
        is_repel: bool,
        is_flutter: bool,
    ) -> Self {
        EntityBoneSnapshot {
            vector,
            rest_length,
            is_repel,
            is_flutter,
            adjustment_strength,
            endurance,
            bias,
        }
    }

    pub fn vector(&self) -> Vector2Df {
        self.vector
    }

    pub fn is_flutter(&self) -> bool {
        self.is_flutter
    }

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
        let bone = EntityBoneSnapshot::new(Vector2Df::zero(), 1.0, 1.0, 1.0, 1.0, false, false);
        assert!(
            bone.get_percent_adjustment() == 0.0,
            "bone of length zero should have no adjustment"
        );
        let bone = EntityBoneSnapshot::new(Vector2Df::up() * 2.0, 1.0, 1.0, 1.0, 1.0, false, false);
        assert!(
            bone.get_percent_adjustment() == 0.5,
            "bone adjustment should be correct"
        );
        let bone = EntityBoneSnapshot::new(Vector2Df::up() * 0.5, 1.0, 1.0, 1.0, 1.0, true, false);
        assert!(
            bone.get_percent_adjustment() != 0.0,
            "repel bone should repel when under rest length"
        );
        let bone = EntityBoneSnapshot::new(Vector2Df::up() * 2.0, 1.0, 1.0, 1.0, 1.0, true, false);
        assert!(
            bone.get_percent_adjustment() == 0.0,
            "repel bone should not repel when over rest length"
        );
        let bone = EntityBoneSnapshot::new(Vector2Df::up(), 1.0, 1.0, 1.0, 5.0, false, false);
        assert!(
            bone.get_percent_adjustment() == -4.0,
            "large rest length factor should give correct result"
        );
        let bone = EntityBoneSnapshot::new(Vector2Df::up(), 1.0, 1.0, 1.0, 0.25, false, false);
        assert!(
            bone.get_percent_adjustment() == 0.75,
            "small rest length factor should give correct result"
        );
    }

    #[test]
    fn get_adjustment() {
        let bone = EntityBoneSnapshot::new(Vector2Df::up() * 2.0, 1.0, 0.5, 1.0, 1.0, false, false);
        assert!(
            bone.get_adjustment() == (-0.5 * Vector2Df::up(), -0.5 * Vector2Df::up()),
            "bone with half bias should adjust both equally"
        );
        let bone = EntityBoneSnapshot::new(Vector2Df::up() * 2.0, 1.0, 0.0, 1.0, 1.0, false, false);
        assert!(
            bone.get_adjustment() == (-1.0 * Vector2Df::up(), 0.0 * Vector2Df::up()),
            "bone with zero bias should only adjust first value"
        );
        let bone = EntityBoneSnapshot::new(Vector2Df::up() * 2.0, 1.0, 1.0, 1.0, 1.0, false, false);
        assert!(
            bone.get_adjustment() == (0.0 * Vector2Df::up(), -1.0 * Vector2Df::up()),
            "bone with one bias should only adjust second value"
        );
        let bone = EntityBoneSnapshot::new(Vector2Df::zero(), 1.0, 1.0, 1.0, 1.0, false, false);
        assert!(
            bone.get_adjustment() == (Vector2Df::zero(), Vector2Df::zero()),
            "bone with current length 0 should have no adjustment"
        );
        let bone = EntityBoneSnapshot::new(Vector2Df::up() * 2.0, 1.0, 1.0, 1.0, 1.0, true, false);
        assert!(
            bone.get_adjustment() == (Vector2Df::zero(), Vector2Df::zero()),
            "repel bone over rest length should not have adjustment"
        );
        let bone = EntityBoneSnapshot::new(Vector2Df::up(), 1.0, 1.0, 1.0, 1.0, false, false);
        assert!(
            bone.get_adjustment() == (Vector2Df::zero(), Vector2Df::zero()),
            "repel bone over rest length should not have adjustment"
        );
        let bone = EntityBoneSnapshot::new(Vector2Df::up(), 1.0, 0.5, 1.0, 5.0, false, false);
        assert!(
            bone.get_adjustment() == (2.0 * Vector2Df::up(), 2.0 * Vector2Df::up()),
            "large rest length factor should give correct result"
        );
        let bone = EntityBoneSnapshot::new(Vector2Df::up(), 1.0, 0.5, 1.0, 0.25, false, false);
        assert!(
            bone.get_adjustment() == (-0.375 * Vector2Df::up(), -0.375 * Vector2Df::up()),
            "small rest length factor should give correct result"
        );
        let bone = EntityBoneSnapshot::new(Vector2Df::up() * 2.0, 6.0, 0.5, 1.0, 1.0, false, false);
        assert!(
            bone.get_adjustment() == (-3.0 * Vector2Df::up(), -3.0 * Vector2Df::up()),
            "adjustment strength should scale adjustment"
        );
    }

    #[test]
    fn get_intact() {
        let bone = EntityBoneSnapshot::new(Vector2Df::zero(), 1.0, 0.5, 1.0, 1.0, false, false);
        assert!(bone.get_intact(), "length 0 bone should be intact");
        let bone = EntityBoneSnapshot::new(Vector2Df::one() * 2.0, 1.0, 0.5, 1.0, 1.0, true, false);
        assert!(
            bone.get_intact(),
            "repel bone over rest length should be intact"
        );
        let bone = EntityBoneSnapshot::new(
            Vector2Df::one() * 5.0,
            1.0,
            0.5,
            INFINITY,
            1.0,
            false,
            false,
        );
        assert!(bone.get_intact(), "infinite endurance should be intact");
        let bone =
            EntityBoneSnapshot::new(Vector2Df::one() * 5.0, 1.0, 0.5, 0.25, 1.0, false, false);
        assert!(!bone.get_intact(), "small endurance should not be intact");
    }
}
