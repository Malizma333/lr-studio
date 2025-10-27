use geometry::Point;
use vector2d::Vector2Df;

pub(crate) struct EntityPointState {
    pub(super) position: Point,
    pub(super) velocity: Vector2Df,
    pub(super) previous_position: Point,
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
