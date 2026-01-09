use crate::entity::point::{entity::EntityPoint, state::EntityPointState};
use geometry::{Line, Point};
use vector2d::Vector2Df;

pub struct PhysicsLine {
    endpoints: Line,
    flipped: bool,
    left_extension: bool,
    right_extension: bool,
    height: f64,
    acceleration_multiplier: f64,

    // Computed props
    inverse_length_squared: f64,
    normal_unit: Vector2Df,
    left_limit: f64,
    right_limit: f64,
    acceleration_vector: Vector2Df,
}

impl PhysicsLine {
    pub fn endpoints(&self) -> Line {
        self.endpoints
    }

    pub fn flipped(&self) -> bool {
        self.flipped
    }

    pub fn left_extension(&self) -> bool {
        self.left_extension
    }

    pub fn right_extension(&self) -> bool {
        self.right_extension
    }

    pub fn height(&self) -> f64 {
        self.height
    }

    pub fn acceleration_multiplier(&self) -> f64 {
        self.acceleration_multiplier
    }

    pub(crate) fn check_interaction(
        &self,
        point: &EntityPoint,
        point_state: &EntityPointState,
    ) -> Option<(Point, Point)> {
        if !point.can_collide() {
            return None;
        }

        let offset_from_point = point_state.position().vector_from(self.endpoints.p0());
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
            let new_position = point_state
                .position()
                .translated_by(-1.0 * self.normal_unit * distance_from_line_top);

            let friction_x_flipped = if point_state.external_velocity().x() >= new_position.x() {
                -1.0
            } else {
                1.0
            };

            let friction_y_flipped = if point_state.external_velocity().y() < new_position.y() {
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

            let new_external_velocity = point_state
                .external_velocity()
                .translated_by(friction_vector)
                .translated_by(-1.0 * self.acceleration_vector);

            Some((new_position, new_external_velocity))
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
    acceleration_multiplier: f64,
}

impl PhysicsLineBuilder {
    pub fn new(endpoints: Line) -> PhysicsLineBuilder {
        const DEFAULT_HEIGHT: f64 = 10.0;
        PhysicsLineBuilder {
            endpoints,
            flipped: false,
            left_extension: false,
            right_extension: false,
            height: DEFAULT_HEIGHT,
            acceleration_multiplier: 0.0,
        }
    }

    pub fn endpoints(mut self, endpoints: Line) -> Self {
        self.endpoints = endpoints;
        self
    }

    pub fn flipped(mut self, flipped: bool) -> Self {
        self.flipped = flipped;
        self
    }

    pub fn left_extension(mut self, left_extension: bool) -> Self {
        self.left_extension = left_extension;
        self
    }

    pub fn right_extension(mut self, right_extension: bool) -> Self {
        self.right_extension = right_extension;
        self
    }

    pub fn height(mut self, height: f64) -> Self {
        self.height = height;
        self
    }

    pub fn acceleration_multiplier(mut self, acceleration_multiplier: f64) -> Self {
        self.acceleration_multiplier = acceleration_multiplier;
        self
    }

    pub fn build(self) -> PhysicsLine {
        let vector = self.endpoints.get_vector();
        let length = vector.length();
        let inverse_length_squared = 1.0 / vector.length_squared();
        let unit = vector * (1.0 / length);

        let normal_unit = if self.flipped {
            unit.rotate_cw()
        } else {
            unit.rotate_ccw()
        };

        const MAX_EXTENSION_SIZE: f64 = 0.25;
        let extension_ratio = MAX_EXTENSION_SIZE.min(self.height / length);

        let left_limit = if self.left_extension {
            -extension_ratio
        } else {
            0.0
        };

        let right_limit = if self.right_extension {
            1.0 + extension_ratio
        } else {
            1.0
        };

        const ACCELERATION_FACTOR: f64 = 0.1;
        let acceleration_vector = unit * (self.acceleration_multiplier * ACCELERATION_FACTOR);

        PhysicsLine {
            endpoints: self.endpoints,
            flipped: self.flipped,
            left_extension: self.left_extension,
            right_extension: self.right_extension,
            height: self.height,
            acceleration_multiplier: self.acceleration_multiplier,

            inverse_length_squared,
            normal_unit,
            left_limit,
            right_limit,
            acceleration_vector,
        }
    }
}

impl From<PhysicsLine> for PhysicsLineBuilder {
    fn from(line: PhysicsLine) -> Self {
        PhysicsLineBuilder {
            endpoints: line.endpoints,
            flipped: line.flipped,
            left_extension: line.left_extension,
            right_extension: line.right_extension,
            height: line.height,
            acceleration_multiplier: line.acceleration_multiplier,
        }
    }
}
