use geometry::Point;
use vector2d::Vector2Df;

use crate::{
    ComputedLineProperties, ComputedProperties, Hitbox,
    entity::point::{entity::EntityPoint, state::EntityPointState},
};

pub struct NormalLine {
    endpoints: (Point, Point),
    flipped: bool,
    left_extension: bool,
    right_extension: bool,
}

impl NormalLine {
    pub fn new(
        endpoints: (Point, Point),
        flipped: bool,
        left_extension: bool,
        right_extension: bool,
    ) -> Self {
        NormalLine {
            endpoints,
            flipped,
            left_extension,
            right_extension,
        }
    }
}

impl ComputedLineProperties for NormalLine {
    fn properties(&self) -> ComputedProperties {
        ComputedProperties::new(
            self.endpoints,
            self.flipped,
            self.left_extension,
            self.right_extension,
        )
    }
}

impl Hitbox for NormalLine {
    fn interact(
        &self,
        point: &EntityPoint,
        point_state: &EntityPointState,
        distance_from_line_top: f64,
        _position_between_ends: f64,
    ) -> (Point, Point) {
        let new_position = point_state.position() - (self.normal_unit() * distance_from_line_top);

        let friction_x_flipped = if point_state.previous_position().x() >= new_position.x() {
            -1.0
        } else {
            1.0
        };

        let friction_y_flipped = if point_state.previous_position().y() < new_position.y() {
            -1.0
        } else {
            1.0
        };

        let initial_friction_vector =
            (self.normal_unit().rotate_cw() * point.contact_friction()) * distance_from_line_top;

        let friction_vector = Vector2Df::new(
            friction_x_flipped * initial_friction_vector.x(),
            friction_y_flipped * initial_friction_vector.y(),
        );

        let new_previous_position = point_state.previous_position() + friction_vector;

        (new_position, new_previous_position)
    }
}
