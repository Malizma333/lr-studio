use geometry::Point;

use crate::engine::entity_registry::point::entity::EntityPoint;

pub(crate) struct EntityPointTemplate {
    pub(super) initial_position: Point,
    pub(super) is_contact: bool,
    pub(super) contact_friction: f64,
    pub(super) air_friction: f64,
}

impl EntityPointTemplate {
    pub(crate) fn is_contact(&self) -> bool {
        self.is_contact
    }

    pub(crate) fn build(&self) -> EntityPoint {
        EntityPoint {
            initial_position: self.initial_position,
            is_contact: self.is_contact,
            contact_friction: self.contact_friction,
            air_friction: self.air_friction,
        }
    }
}
