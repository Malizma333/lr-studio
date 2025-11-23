use geometry::Point;
use vector2d::Vector2Df;

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
