use geometry::Point;
use vector2d::Vector2Df;

use crate::entity::point::{EntityPoint, EntityPointState};

pub(crate) struct EntityPointTemplate {
    initial_position: Point,
    contact: bool,
    contact_friction: Option<f64>,
    air_friction: Option<f64>,
}

impl EntityPointTemplate {
    pub fn new(initial_position: Point) -> EntityPointTemplate {
        EntityPointTemplate {
            initial_position,
            contact: false,
            contact_friction: None,
            air_friction: None,
        }
    }

    pub fn contact(&mut self) -> &mut Self {
        self.contact = true;
        self
    }

    pub fn contact_friction(&mut self, friction: f64) -> &mut Self {
        self.contact_friction = Some(friction);
        self
    }

    pub fn air_friction(&mut self, friction: f64) -> &mut Self {
        self.air_friction = Some(friction);
        self
    }

    pub fn build(&self) -> EntityPoint {
        EntityPoint {
            contact: self.contact,
            contact_friction: self.contact_friction.unwrap_or(0.0),
            air_friction: self.air_friction.unwrap_or(0.0),
            state: EntityPointState {
                position: self.initial_position,
                velocity: Vector2Df::zero(),
                previous_position: self.initial_position,
            },
        }
    }
}
