use geometry::{Line, Point};
use vector2d::Vector2Df;

use crate::entity::point::{entity::EntityPoint, state::EntityPointState};

const DEFAULT_HEIGHT: f64 = 10.0;

pub struct PhysicsLine {
    endpoints: Line,
    height: f64,
    inverse_length_squared: f64,
    normal_unit: Vector2Df,
    left_limit: f64,
    right_limit: f64,
    acceleration_vector: Vector2Df,
}

impl PhysicsLine {
    pub(crate) fn endpoints(&self) -> Line {
        self.endpoints
    }

    /** Returns the new (position, previous position) to update a point with after it interacts with this line\
    (The previous position is not necessarily `position - velocity`, it represents how much force is applied
    on the momentum tick due to forces such as friction)
    */
    fn interact(
        &self,
        point: &EntityPoint,
        point_state: &EntityPointState,
        distance_from_line_top: f64,
        _position_between_ends: f64,
    ) -> (Point, Point) {
        let new_position = point_state.position() - (self.normal_unit * distance_from_line_top);

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
            (self.normal_unit.rotate_cw() * point.contact_friction()) * distance_from_line_top;

        let friction_vector = Vector2Df::new(
            friction_x_flipped * initial_friction_vector.x(),
            friction_y_flipped * initial_friction_vector.y(),
        );

        let new_previous_position =
            point_state.previous_position() + friction_vector - self.acceleration_vector;

        (new_position, new_previous_position)
    }

    pub(crate) fn check_interaction(
        &self,
        point: &EntityPoint,
        point_state: &EntityPointState,
    ) -> Option<(Point, Point)> {
        if !point.is_contact() {
            return None;
        }

        let offset_from_point = point_state.position() - self.endpoints.p0();
        let moving_into_line = Vector2Df::dot(self.normal_unit, point_state.velocity()) > 0.0;
        let distance_from_line_top = Vector2Df::dot(self.normal_unit, offset_from_point);
        let position_between_ends = Vector2Df::dot(self.endpoints.get_vector(), offset_from_point)
            * self.inverse_length_squared;

        if moving_into_line
            && 0.0 < distance_from_line_top
            && distance_from_line_top < self.height
            && self.left_limit <= position_between_ends
            && position_between_ends <= self.right_limit
        {
            Some(self.interact(
                point,
                point_state,
                distance_from_line_top,
                position_between_ends,
            ))
        } else {
            None
        }
    }
}

pub struct PhysicsLineBuilder {
    endpoints: Line,
    flipped: bool,
    left_extension: bool,
    right_extension: bool,
    height: f64,
    multiplier: f64,
}

impl PhysicsLineBuilder {
    pub fn new(endpoints: Line) -> Self {
        Self {
            endpoints,
            flipped: false,
            left_extension: false,
            right_extension: false,
            height: DEFAULT_HEIGHT,
            multiplier: 0.0,
        }
    }

    pub fn flipped(&mut self, flipped: bool) -> &mut Self {
        self.flipped = flipped;
        self
    }

    pub fn left_extension(&mut self, left_extension: bool) -> &mut Self {
        self.left_extension = left_extension;
        self
    }

    pub fn right_extension(&mut self, right_extension: bool) -> &mut Self {
        self.right_extension = right_extension;
        self
    }

    pub fn height(&mut self, height: f64) -> &mut Self {
        self.height = height;
        self
    }

    pub fn acceleration_multiplier(&mut self, multiplier: f64) -> &mut Self {
        self.multiplier = multiplier;
        self
    }

    pub fn build(&self) -> PhysicsLine {
        let endpoints = self.endpoints;
        let height = self.height;
        let flipped = self.flipped;
        let left_extension = self.left_extension;
        let right_extension = self.right_extension;
        let multiplier = self.multiplier;

        let vector = endpoints.get_vector();
        let length = vector.length();
        let inverse_length_squared = 1.0 / vector.length_squared();
        let unit = vector * (1.0 / length);

        let normal_unit = if flipped {
            unit.rotate_cw()
        } else {
            unit.rotate_ccw()
        };

        const MAX_EXTENSION_SIZE: f64 = 0.25;
        let extension_ratio = MAX_EXTENSION_SIZE.min(height / length);

        let left_limit = if left_extension {
            -extension_ratio
        } else {
            0.0
        };

        let right_limit = if right_extension {
            1.0 + extension_ratio
        } else {
            1.0
        };

        const ACCELERATION_FACTOR: f64 = 0.1;
        let acceleration_vector = unit * (multiplier * ACCELERATION_FACTOR);

        PhysicsLine {
            endpoints: self.endpoints,
            height: self.height,
            inverse_length_squared,
            normal_unit,
            left_limit,
            right_limit,
            acceleration_vector,
        }
    }
}
