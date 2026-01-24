use std::collections::BTreeMap;

use vector2d::Vector2Df;

use crate::entity_registry::{
    EntityPoint, EntityPointId, EntityState,
    entity_template::{MountId, SegmentId},
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum ConnectionType {
    Segment(SegmentId),
    Mount(MountId),
}

/// Computed properties when built
struct Computed {
    rest_length: f64,
    is_flutter: bool,
    connection_type: ConnectionType,
}

/// Constructed bone that holds props after building
pub struct EntityBone {
    point_ids: (EntityPointId, EntityPointId),
    bias: f64,
    initial_length_factor: f64,
    repel_only: bool,
    endurance: f64,
    adjustment_strength: f64,
    endurance_remount_factor: f64,
    adjustment_strength_remount_factor: f64,
    computed: Computed,
}

impl EntityBone {
    pub(crate) fn point_ids(&self) -> (EntityPointId, EntityPointId) {
        self.point_ids
    }

    pub(crate) fn is_flutter(&self) -> bool {
        self.computed.is_flutter
    }

    pub(crate) fn connection_type(&self) -> ConnectionType {
        self.computed.connection_type
    }

    pub(crate) fn get_percent_adjustment(&self, bone_vector: Vector2Df) -> f64 {
        let current_length = bone_vector.length();
        let should_repel = current_length < self.computed.rest_length;

        if current_length == 0.0 || (self.repel_only && !should_repel) {
            0.0
        } else {
            (current_length - self.computed.rest_length) / current_length
        }
    }

    pub(crate) fn adjust_points(&self, state: &mut EntityState, remounting: bool) {
        let point_states = (
            state.point_state(&self.point_ids.0),
            state.point_state(&self.point_ids.1),
        );

        let bone_vector = point_states
            .0
            .position()
            .vector_from(point_states.1.position());
        let percent_adjustment = self.get_percent_adjustment(bone_vector);

        let adjustment_strength = if remounting {
            self.adjustment_strength * self.adjustment_strength_remount_factor
        } else {
            self.adjustment_strength
        };
        let adjustment = adjustment_strength * percent_adjustment;

        let adjusted = (
            point_states
                .0
                .position()
                .translated_by(-bone_vector * adjustment * (1.0 - self.bias)),
            point_states
                .1
                .position()
                .translated_by(bone_vector * adjustment * self.bias),
        );

        state
            .point_state_mut(&self.point_ids.0)
            .update(Some(adjusted.0), None, None);
        state
            .point_state_mut(&self.point_ids.1)
            .update(Some(adjusted.1), None, None);
    }

    pub(crate) fn is_intact(&self, state: &EntityState, remounting: bool) -> bool {
        let point_states = (
            state.point_state(&self.point_ids.0),
            state.point_state(&self.point_ids.1),
        );
        let bone_vector = point_states
            .1
            .position()
            .vector_from(point_states.0.position());
        let percent_adjustment = self.get_percent_adjustment(bone_vector);

        let endurance = if remounting {
            self.endurance * self.endurance_remount_factor
        } else {
            self.endurance
        };

        percent_adjustment <= endurance * self.computed.rest_length
    }
}

/// Bone builder for custom skeletons
pub struct EntityBoneBuilder {
    point_ids: (EntityPointId, EntityPointId),
    bias: f64,
    initial_length_factor: f64,
    repel_only: bool,
    endurance: f64,
    adjustment_strength: f64,
    endurance_remount_factor: f64,
    adjustment_strength_remount_factor: f64,
}

impl EntityBoneBuilder {
    pub fn new(point_ids: (EntityPointId, EntityPointId)) -> EntityBoneBuilder {
        Self {
            point_ids,
            bias: 0.5,
            initial_length_factor: 1.0,
            repel_only: false,
            endurance: f64::INFINITY,
            adjustment_strength: 1.0,
            endurance_remount_factor: 1.0,
            adjustment_strength_remount_factor: 1.0,
        }
    }

    pub(crate) fn point_ids(&self) -> (EntityPointId, EntityPointId) {
        self.point_ids
    }

    pub(crate) fn breakable(&self) -> bool {
        self.endurance != f64::INFINITY
    }

    pub fn bias(mut self, bias: f64) -> Self {
        self.bias = bias;
        self
    }

    pub fn initial_length_factor(mut self, rest_length_factor: f64) -> Self {
        self.initial_length_factor = rest_length_factor;
        self
    }

    pub fn repel(mut self, repel_only: bool) -> Self {
        self.repel_only = repel_only;
        self
    }

    pub fn endurance(mut self, endurance: f64) -> Self {
        self.endurance = endurance.max(0.0);
        self
    }

    pub fn adjustment_strength(mut self, strength: f64) -> Self {
        self.adjustment_strength = strength;
        self
    }

    pub fn endurance_remount_factor(mut self, factor: f64) -> Self {
        self.endurance_remount_factor = factor;
        self
    }

    pub fn adjustment_strength_remount_factor(mut self, factor: f64) -> Self {
        self.adjustment_strength_remount_factor = factor;
        self
    }

    pub(crate) fn build(
        self,
        point_map: &BTreeMap<EntityPointId, EntityPoint>,
        connection_type: ConnectionType,
    ) -> EntityBone {
        let points = (
            point_map
                .get(&self.point_ids.0)
                .expect("Building this bone should properly resolve builder points"),
            point_map
                .get(&self.point_ids.1)
                .expect("Building this bone should properly resolve builder points"),
        );

        let is_flutter = !(points.0.is_contact() && points.1.is_contact());
        let rest_length = points
            .0
            .initial_position()
            .distance_from(points.1.initial_position())
            * self.initial_length_factor;

        EntityBone {
            point_ids: self.point_ids,
            bias: self.bias,
            repel_only: self.repel_only,
            initial_length_factor: self.initial_length_factor,
            endurance: self.endurance,
            adjustment_strength: self.adjustment_strength,
            endurance_remount_factor: self.endurance_remount_factor,
            adjustment_strength_remount_factor: self.adjustment_strength_remount_factor,
            computed: Computed {
                rest_length,
                is_flutter,
                connection_type,
            },
        }
    }
}

impl From<EntityBone> for EntityBoneBuilder {
    fn from(bone: EntityBone) -> Self {
        Self {
            point_ids: bone.point_ids,
            bias: bone.bias,
            initial_length_factor: bone.initial_length_factor,
            repel_only: bone.repel_only,
            endurance: bone.endurance,
            adjustment_strength: bone.adjustment_strength,
            endurance_remount_factor: bone.endurance_remount_factor,
            adjustment_strength_remount_factor: bone.adjustment_strength_remount_factor,
        }
    }
}
