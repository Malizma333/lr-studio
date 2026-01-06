use geometry::Point;
use vector2d::Vector2Df;

use crate::entity::point::state::EntityPointState;

pub struct EntityPoint {
    pub(super) initial_position: Point,
    pub(super) contact: bool,
    pub(super) contact_friction: f64,
    pub(super) air_friction: f64,
}

impl EntityPoint {
    pub(crate) fn initial_position(&self) -> Point {
        self.initial_position
    }

    pub(crate) fn is_contact(&self) -> bool {
        self.contact
    }

    pub(crate) fn contact_friction(&self) -> f64 {
        self.contact_friction
    }

    pub(crate) fn process_initial_step(&self, state: &mut EntityPointState, gravity: Vector2Df) {
        let computed_velocity = state.position() - state.previous_position();
        let new_velocity = computed_velocity * (1.0 - self.air_friction) + gravity;
        let new_position = state.position() + new_velocity;
        state.update(
            Some(new_position),
            Some(new_velocity),
            Some(state.position()),
        );
    }
}
