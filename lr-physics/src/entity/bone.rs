use std::f64::INFINITY;

use vector2d::Vector2Df;

use crate::engine::entity_registry::{EntityRegistry, EntityRegistryIndex};

pub struct EntityBone {
    connected_points: (EntityRegistryIndex, EntityRegistryIndex),
    bias: f64,
    initial_length: f64,
    initial_length_factor: f64,
    repel_only: bool,
    endurance: f64,
    adjustment_strength: f64,
    endurance_remount_factor: f64,
    adjustment_strength_remount_factor: f64,
}

pub struct EntityBoneBuilder {
    connected_points: Option<(EntityRegistryIndex, EntityRegistryIndex)>,
    bias: Option<f64>,
    initial_length_factor: Option<f64>,
    repel_only: bool,
    endurance: Option<f64>,
    adjustment_strength: Option<f64>,
    endurance_remount_factor: Option<f64>,
    adjustment_strength_remount_factor: Option<f64>,
}

#[derive(Debug, Clone)]
pub enum EntityBoneBuilderError {
    MissingPoints,
}

impl EntityBoneBuilder {
    pub fn new() -> EntityBoneBuilder {
        EntityBoneBuilder {
            connected_points: None,
            bias: None,
            initial_length_factor: None,
            repel_only: false,
            endurance: None,
            adjustment_strength: None,
            endurance_remount_factor: None,
            adjustment_strength_remount_factor: None,
        }
    }

    pub fn points(&mut self, p1: EntityRegistryIndex, p2: EntityRegistryIndex) -> &mut Self {
        self.connected_points = Some((p1, p2));
        self
    }

    pub fn bias(&mut self, bias: f64) -> &mut Self {
        self.bias = Some(bias);
        self
    }

    pub fn initial_length_factor(&mut self, rest_length_factor: f64) -> &mut Self {
        self.initial_length_factor = Some(rest_length_factor);
        self
    }

    pub fn repel(&mut self) -> &mut Self {
        self.repel_only = true;
        self
    }

    pub fn endurance(&mut self, endurance: f64) -> &mut Self {
        self.endurance = Some(endurance);
        self
    }

    pub fn adjustment_strength(&mut self, strength: f64) -> &mut Self {
        self.adjustment_strength = Some(strength);
        self
    }

    pub fn endurance_remount_factor(&mut self, factor: f64) -> &mut Self {
        self.endurance_remount_factor = Some(factor);
        self
    }

    pub fn adjustment_strength_remount_factor(&mut self, factor: f64) -> &mut Self {
        self.adjustment_strength_remount_factor = Some(factor);
        self
    }

    pub fn build(&self, registry: &EntityRegistry) -> Result<EntityBone, EntityBoneBuilderError> {
        if let Some(connected_points) = self.connected_points {
            let bone_vector = registry.get_point(connected_points.1).position()
                - registry.get_point(connected_points.0).position();
            Ok(EntityBone {
                connected_points,
                bias: self.bias.unwrap_or(0.5),
                initial_length: bone_vector.length(),
                initial_length_factor: self.initial_length_factor.unwrap_or(1.0),
                repel_only: self.repel_only,
                endurance: self.endurance.unwrap_or(INFINITY),
                adjustment_strength: self.adjustment_strength.unwrap_or(1.0),
                endurance_remount_factor: self.endurance_remount_factor.unwrap_or(1.0),
                adjustment_strength_remount_factor: self.endurance_remount_factor.unwrap_or(1.0),
            })
        } else {
            Err(EntityBoneBuilderError::MissingPoints)
        }
    }
}

pub trait EntityBoneLogic {
    fn vector(&self) -> Vector2Df;
    fn rest_length(&self) -> f64;
    fn is_repel(&self) -> bool;
    fn adjustment_strength(&self) -> f64;
    fn endurance(&self) -> f64;
    fn bias(&self) -> f64;

    fn get_percent_adjustment(&self) -> f64 {
        let bone_vector = self.vector();
        let current_length = bone_vector.length();
        let should_repel = current_length < self.rest_length();

        if current_length == 0.0 || (self.is_repel() && !should_repel) {
            0.0
        } else {
            (current_length - self.rest_length()) / current_length
        }
    }

    fn get_adjustment(&self) -> (Vector2Df, Vector2Df) {
        let bone_vector = self.vector();
        let percent_adjustment = self.get_percent_adjustment();
        let adjustment_strength = self.adjustment_strength();
        (
            -1.0 * bone_vector * adjustment_strength * percent_adjustment * (1.0 - self.bias()),
            -1.0 * bone_vector * adjustment_strength * percent_adjustment * self.bias(),
        )
    }

    fn get_intact(&self) -> bool {
        let percent_adjustment = self.get_percent_adjustment();
        let endurance = self.endurance();
        percent_adjustment <= endurance * self.rest_length()
    }
}

// Entity bone with references to contact points resolved
pub struct EntityBoneSnapshot {
    vector: Vector2Df,
    rest_length: f64,
    is_repel: bool,
    is_flutter: bool,
    adjustment_strength: f64,
    endurance: f64,
    bias: f64,
}

impl EntityBoneLogic for EntityBoneSnapshot {
    fn vector(&self) -> Vector2Df {
        self.vector
    }

    fn rest_length(&self) -> f64 {
        self.rest_length
    }

    fn is_repel(&self) -> bool {
        self.is_repel
    }

    fn adjustment_strength(&self) -> f64 {
        self.adjustment_strength
    }

    fn endurance(&self) -> f64 {
        self.endurance
    }

    fn bias(&self) -> f64 {
        self.bias
    }
}

impl EntityBoneSnapshot {
    pub fn is_flutter(&self) -> bool {
        self.is_flutter
    }
}

impl EntityBone {
    pub fn get_snapshot(&self, registry: &EntityRegistry, remounting: bool) -> EntityBoneSnapshot {
        let is_flutter = !(registry.get_point(self.connected_points.0).is_contact()
            && registry.get_point(self.connected_points.1).is_contact());
        let vector = registry.get_point(self.connected_points.1).position()
            - registry.get_point(self.connected_points.0).position();
        let adjustment_strength = if remounting {
            self.adjustment_strength * self.adjustment_strength_remount_factor
        } else {
            self.adjustment_strength
        };
        let endurance = if remounting {
            self.endurance * self.endurance_remount_factor
        } else {
            self.endurance
        };
        let rest_length = self.initial_length * self.initial_length_factor;

        EntityBoneSnapshot {
            vector,
            rest_length,
            is_flutter,
            adjustment_strength,
            endurance,
            is_repel: self.repel_only,
            bias: self.bias,
        }
    }

    pub fn get_points(&self) -> (EntityRegistryIndex, EntityRegistryIndex) {
        self.connected_points
    }
}

#[cfg(test)]
mod tests {
    use std::f64::INFINITY;

    use vector2d::Vector2Df;

    use crate::entity::bone::EntityBoneLogic;

    struct PureBone(pub Vector2Df, pub f64, pub f64, pub f64, pub f64, pub bool);

    impl EntityBoneLogic for PureBone {
        fn vector(&self) -> Vector2Df {
            self.0
        }

        fn adjustment_strength(&self) -> f64 {
            self.1
        }

        fn bias(&self) -> f64 {
            self.2
        }

        fn endurance(&self) -> f64 {
            self.3
        }

        fn rest_length(&self) -> f64 {
            self.4
        }

        fn is_repel(&self) -> bool {
            self.5
        }
    }

    #[test]
    fn get_percent_adjustment() {
        let bone = PureBone(Vector2Df::zero(), 1.0, 1.0, 1.0, 1.0, false);
        assert!(
            bone.get_percent_adjustment() == 0.0,
            "bone of length zero should have no adjustment"
        );
        let bone = PureBone(Vector2Df::up() * 2.0, 1.0, 1.0, 1.0, 1.0, false);
        assert!(
            bone.get_percent_adjustment() == 0.5,
            "bone adjustment should be correct"
        );
        let bone = PureBone(Vector2Df::up() * 0.5, 1.0, 1.0, 1.0, 1.0, true);
        assert!(
            bone.get_percent_adjustment() != 0.0,
            "repel bone should repel when under rest length"
        );
        let bone = PureBone(Vector2Df::up() * 2.0, 1.0, 1.0, 1.0, 1.0, true);
        assert!(
            bone.get_percent_adjustment() == 0.0,
            "repel bone should not repel when over rest length"
        );
        let bone = PureBone(Vector2Df::up(), 1.0, 1.0, 1.0, 5.0, false);
        assert!(
            bone.get_percent_adjustment() == -4.0,
            "large rest length factor should give correct result"
        );
        let bone = PureBone(Vector2Df::up(), 1.0, 1.0, 1.0, 0.25, false);
        assert!(
            bone.get_percent_adjustment() == 0.75,
            "small rest length factor should give correct result"
        );
    }

    #[test]
    fn get_adjustment() {
        let bone = PureBone(Vector2Df::up() * 2.0, 1.0, 0.5, 1.0, 1.0, false);
        assert!(
            bone.get_adjustment() == (-0.5 * Vector2Df::up(), -0.5 * Vector2Df::up()),
            "bone with half bias should adjust both equally"
        );
        let bone = PureBone(Vector2Df::up() * 2.0, 1.0, 0.0, 1.0, 1.0, false);
        assert!(
            bone.get_adjustment() == (-1.0 * Vector2Df::up(), 0.0 * Vector2Df::up()),
            "bone with zero bias should only adjust first value"
        );
        let bone = PureBone(Vector2Df::up() * 2.0, 1.0, 1.0, 1.0, 1.0, false);
        assert!(
            bone.get_adjustment() == (0.0 * Vector2Df::up(), -1.0 * Vector2Df::up()),
            "bone with one bias should only adjust second value"
        );
        let bone = PureBone(Vector2Df::zero(), 1.0, 1.0, 1.0, 1.0, false);
        assert!(
            bone.get_adjustment() == (Vector2Df::zero(), Vector2Df::zero()),
            "bone with current length 0 should have no adjustment"
        );
        let bone = PureBone(Vector2Df::up() * 2.0, 1.0, 1.0, 1.0, 1.0, true);
        assert!(
            bone.get_adjustment() == (Vector2Df::zero(), Vector2Df::zero()),
            "repel bone over rest length should not have adjustment"
        );
        let bone = PureBone(Vector2Df::up(), 1.0, 1.0, 1.0, 1.0, false);
        assert!(
            bone.get_adjustment() == (Vector2Df::zero(), Vector2Df::zero()),
            "repel bone over rest length should not have adjustment"
        );
        let bone = PureBone(Vector2Df::up(), 1.0, 0.5, 1.0, 5.0, false);
        assert!(
            bone.get_adjustment() == (2.0 * Vector2Df::up(), 2.0 * Vector2Df::up()),
            "large rest length factor should give correct result"
        );
        let bone = PureBone(Vector2Df::up(), 1.0, 0.5, 1.0, 0.25, false);
        assert!(
            bone.get_adjustment() == (-0.375 * Vector2Df::up(), -0.375 * Vector2Df::up()),
            "small rest length factor should give correct result"
        );
        let bone = PureBone(Vector2Df::up() * 2.0, 6.0, 0.5, 1.0, 1.0, false);
        assert!(
            bone.get_adjustment() == (-3.0 * Vector2Df::up(), -3.0 * Vector2Df::up()),
            "adjustment strength should scale adjustment"
        );
    }

    #[test]
    fn get_intact() {
        let bone = PureBone(Vector2Df::zero(), 1.0, 0.5, 1.0, 1.0, false);
        assert!(bone.get_intact(), "length 0 bone should be intact");
        let bone = PureBone(Vector2Df::one() * 2.0, 1.0, 0.5, 1.0, 1.0, true);
        assert!(
            bone.get_intact(),
            "repel bone over rest length should be intact"
        );
        let bone = PureBone(Vector2Df::one() * 5.0, 1.0, 0.5, INFINITY, 1.0, false);
        assert!(bone.get_intact(), "infinite endurance should be intact");
        let bone = PureBone(Vector2Df::one() * 5.0, 1.0, 0.5, 0.25, 1.0, false);
        assert!(!bone.get_intact(), "small endurance should not be intact");
    }
}
