use geometry::Point;
use vector2d::Vector2Df;

#[derive(Debug, Clone)]
pub struct EntityPointState {
    position: Point,
    velocity: Vector2Df,
    external_velocity: Point,
}

impl EntityPointState {
    pub(crate) fn new(position: Point, velocity: Vector2Df, external_velocity: Point) -> Self {
        EntityPointState {
            position,
            velocity,
            external_velocity,
        }
    }

    pub(crate) fn update(
        &mut self,
        new_position: Option<Point>,
        new_velocity: Option<Vector2Df>,
        external_velocity: Option<Point>,
    ) {
        self.position = new_position.unwrap_or(self.position);
        self.velocity = new_velocity.unwrap_or(self.velocity);
        self.external_velocity = external_velocity.unwrap_or(self.external_velocity);
    }

    pub(crate) fn position(&self) -> Point {
        self.position
    }

    pub(crate) fn velocity(&self) -> Vector2Df {
        self.velocity
    }

    pub(crate) fn external_velocity(&self) -> Point {
        self.external_velocity
    }
}
