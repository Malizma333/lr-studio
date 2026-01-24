use geometry::Point;
use vector2d::Vector2Df;

#[derive(Debug, Clone)]
pub(crate) struct EntityPointState {
    position: Point,
    velocity: Vector2Df,
    computed_previous_position: Point,
}

impl EntityPointState {
    pub(super) fn new(
        position: Point,
        velocity: Vector2Df,
        computed_previous_position: Point,
    ) -> Self {
        EntityPointState {
            position,
            velocity,
            computed_previous_position,
        }
    }

    pub(crate) fn update(
        &mut self,
        new_position: Option<Point>,
        new_velocity: Option<Vector2Df>,
        computed_previous_position: Option<Point>,
    ) {
        self.position = new_position.unwrap_or(self.position);
        self.velocity = new_velocity.unwrap_or(self.velocity);
        self.computed_previous_position =
            computed_previous_position.unwrap_or(self.computed_previous_position);
    }

    pub(crate) fn position(&self) -> Point {
        self.position
    }

    pub(crate) fn velocity(&self) -> Vector2Df {
        self.velocity
    }

    pub(crate) fn computed_previous_position(&self) -> Point {
        self.computed_previous_position
    }
}
