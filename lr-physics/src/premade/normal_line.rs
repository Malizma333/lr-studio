use geometry::Point;

use crate::line::{
    computed::{ComputedLineProperties, ComputedProperties},
    hitbox::Hitbox,
};

pub struct NormalLine {
    endpoints: (Point, Point),
    flipped: bool,
    left_extension: bool,
    right_extension: bool,
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
        point: &crate::entity::point::EntityPoint,
        distance_from_line_top: f64,
        _position_between_ends: f64,
    ) -> Option<(Point, Point)> {
        let new_position = point.position() - (self.normal_unit() * distance_from_line_top);

        let mut friction_vector =
            (self.normal_unit().rotate_cw() * point.friction()) * distance_from_line_top;

        if point.previous_position().x >= new_position.x {
            friction_vector.x *= -1.0;
        }

        if point.previous_position().y < new_position.y {
            friction_vector.y *= -1.0;
        }

        let new_previous_position = point.previous_position() + friction_vector;

        Some((new_position, new_previous_position))
    }
}
