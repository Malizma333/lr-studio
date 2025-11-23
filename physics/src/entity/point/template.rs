use geometry::Point;
use vector2d::Vector2Df;

use crate::entity::point::entity::EntityPoint;

pub(crate) struct EntityPointTemplate {
    pub(super) initial_position: Point,
    pub(super) initial_velocity: Vector2Df,
    pub(super) contact: bool,
    pub(super) contact_friction: f64,
    pub(super) air_friction: f64,
}

impl EntityPointTemplate {
    pub(crate) fn build(&self) -> EntityPoint {
        // TODO initial state
        // position: self.initial_position,
        // velocity: self.initial_velocity,
        // previous_position: self.initial_position,
        EntityPoint {
            contact: self.contact,
            contact_friction: self.contact_friction,
            air_friction: self.air_friction,
        }
    }
}
