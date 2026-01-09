use geometry::Point;

pub struct EntityPoint {
    pub(super) initial_position: Point,
    pub(super) is_contact: bool,
    pub(super) contact_friction: f64,
    pub(super) air_friction: f64,
}

impl EntityPoint {
    pub(crate) fn initial_position(&self) -> Point {
        self.initial_position
    }

    pub(crate) fn can_collide(&self) -> bool {
        self.is_contact
    }

    pub(crate) fn contact_friction(&self) -> f64 {
        self.contact_friction
    }

    pub(crate) fn air_friction(&self) -> f64 {
        self.air_friction
    }
}
