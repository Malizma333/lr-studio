use geometry::Point;
use vector2d::Vector2Df;

#[derive(Debug)]
pub struct EntityPointState {
    position: Point,
    velocity: Vector2Df,
    previous_position: Point,
}

impl Clone for EntityPointState {
    fn clone(&self) -> Self {
        Self {
            position: self.position.clone(),
            velocity: self.velocity.clone(),
            previous_position: self.previous_position.clone(),
        }
    }
}

impl EntityPointState {
    pub(crate) fn new(position: Point, velocity: Vector2Df, previous_position: Point) -> Self {
        EntityPointState {
            position,
            velocity,
            previous_position,
        }
    }

    pub(crate) fn update(
        &mut self,
        new_position: Option<Point>,
        new_velocity: Option<Vector2Df>,
        new_previous_position: Option<Point>,
    ) {
        self.position = new_position.unwrap_or(self.position);
        self.velocity = new_velocity.unwrap_or(self.velocity);
        self.previous_position = new_previous_position.unwrap_or(self.previous_position);
    }

    pub(crate) fn position(&self) -> Point {
        self.position
    }

    pub(crate) fn velocity(&self) -> Vector2Df {
        self.velocity
    }

    pub(crate) fn previous_position(&self) -> Point {
        self.previous_position
    }
}
