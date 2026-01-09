use geometry::Point;
use vector2d::Vector2Df;

use crate::engine::entity_registry::{EntityPointId, point::state::EntityPointState};

pub(crate) struct EntityBone {
    pub(super) points: (EntityPointId, EntityPointId),
    pub(super) rest_length: f64,
    pub(super) bias: f64,
    pub(super) repel_only: bool,
    pub(super) is_flutter: bool,
    pub(super) endurance: f64,
    pub(super) endurance_remount_factor: f64,
    pub(super) adjustment_strength: f64,
    pub(super) adjustment_strength_remount_factor: f64,
}

impl EntityBone {
    pub(crate) fn points(&self) -> (EntityPointId, EntityPointId) {
        self.points
    }

    pub(crate) fn get_percent_adjustment(&self, bone_vector: Vector2Df) -> f64 {
        let current_length = bone_vector.length();
        let should_repel = current_length < self.rest_length;

        if current_length == 0.0 || (self.repel_only && !should_repel) {
            0.0
        } else {
            (current_length - self.rest_length) / current_length
        }
    }

    pub(crate) fn get_adjusted(
        &self,
        point_states: (&EntityPointState, &EntityPointState),
        remounting: bool,
    ) -> (Point, Point) {
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

        (
            point_states
                .0
                .position()
                .translated_by(-bone_vector * adjustment * (1.0 - self.bias)),
            point_states
                .1
                .position()
                .translated_by(bone_vector * adjustment * self.bias),
        )
    }

    pub(crate) fn get_intact(
        &self,
        point_states: (&EntityPointState, &EntityPointState),
        remounting: bool,
    ) -> bool {
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

        percent_adjustment <= endurance * self.rest_length
    }

    pub(crate) fn is_flutter(&self) -> bool {
        self.is_flutter
    }

    pub(crate) fn is_breakable(&self) -> bool {
        self.endurance < f64::INFINITY
    }
}
