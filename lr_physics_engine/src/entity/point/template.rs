use geometry::Point;

use crate::entity::point::entity::EntityPoint;

pub(crate) struct EntityPointTemplate {
    pub(super) initial_position: Point,
    pub(super) contact: bool,
    pub(super) contact_friction: f64,
    pub(super) air_friction: f64,
}

impl EntityPointTemplate {
    pub(crate) fn is_contact(&self) -> bool {
        self.contact
    }

    pub(crate) fn build(&self) -> EntityPoint {
        EntityPoint {
            initial_position: self.initial_position,
            contact: self.contact,
            contact_friction: self.contact_friction,
            air_friction: self.air_friction,
        }
    }
}
