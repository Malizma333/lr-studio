use vector2d::Vector2Df;

use crate::entity::point::snapshot::EntityPointSnapshot;

pub(crate) struct EntityPoint {
    pub(super) contact: bool,
    pub(super) contact_friction: f64,
    pub(super) air_friction: f64,
}

impl EntityPoint {
    pub(crate) fn get_snapshot(&self) -> EntityPointSnapshot {
        // TODO resolve state
        EntityPointSnapshot {
            position: Vector2Df::zero(),
            velocity: Vector2Df::zero(),
            previous_position: Vector2Df::zero(),
            contact_friction: self.contact_friction,
            air_friction: self.air_friction,
            contact: self.contact,
        }
    }
}
