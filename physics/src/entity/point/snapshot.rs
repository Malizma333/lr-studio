use geometry::Point;
use vector2d::Vector2Df;

pub struct EntityPointSnapshot {
    pub(super) contact: bool,
    pub(super) contact_friction: f64,
    pub(super) air_friction: f64,
    pub(super) position: Point,
    pub(super) velocity: Vector2Df,
    pub(super) previous_position: Point,
}

#[cfg(test)]
impl EntityPointSnapshot {
    // Used for testing hitbox-point collisions
    pub(crate) fn new(
        position: Point,
        velocity: Vector2Df,
        previous_position: Point,
        contact_friction: f64,
        air_friction: f64,
        contact: bool,
    ) -> Self {
        Self {
            position,
            velocity,
            previous_position,
            contact_friction,
            air_friction,
            contact,
        }
    }
}

impl EntityPointSnapshot {
    pub fn is_contact(&self) -> bool {
        self.contact
    }

    pub fn contact_friction(&self) -> f64 {
        self.contact_friction
    }

    pub fn air_friction(&self) -> f64 {
        self.air_friction
    }

    pub fn position(&self) -> Point {
        self.position
    }

    pub fn velocity(&self) -> Vector2Df {
        self.velocity
    }

    pub fn previous_position(&self) -> Point {
        self.previous_position
    }

    fn get_initial_step(&self, gravity: Vector2Df) -> (Point, Vector2Df, Point) {
        let computed_velocity = self.position - self.previous_position;
        let new_velocity = computed_velocity * (1.0 - self.air_friction) + gravity;
        let new_position = self.position + new_velocity;
        (new_position, new_velocity, self.position)
    }
}

#[cfg(test)]
mod tests {
    use geometry::Point;
    use vector2d::Vector2Df;

    use crate::entity::point::snapshot::EntityPointSnapshot;

    #[test]
    fn initial_step_zero_gravity() {
        let point = EntityPointSnapshot {
            position: Point::zero(),
            velocity: Vector2Df::zero(),
            previous_position: Vector2Df::zero(),
            contact_friction: 0.0,
            air_friction: 0.0,
            contact: true,
        };
        let result = point.get_initial_step(Vector2Df::zero());
        assert!(result.0 == Point::zero(), "Position should be zero");
        assert!(result.1 == Vector2Df::zero(), "Velocity should be zero");
        assert!(
            result.2 == Vector2Df::zero(),
            "Previous should copy last position"
        );

        let point = EntityPointSnapshot {
            position: Point::up(),
            velocity: Point::up(),
            previous_position: Point::zero(),
            contact_friction: 0.0,
            air_friction: 0.0,
            contact: true,
        };

        let result = point.get_initial_step(Vector2Df::zero());
        assert!(result.0 == Point::up() * 2.0, "Position should increase");
        assert!(result.1 == Vector2Df::up(), "Velocity should stay the same");
        assert!(
            result.2 == Vector2Df::up(),
            "Previous should copy last position"
        );
    }

    #[test]
    fn initial_step_normal_gravity() {
        let point = EntityPointSnapshot {
            position: Point::up(),
            velocity: Point::up(),
            previous_position: Point::zero(),
            contact_friction: 0.0,
            air_friction: 0.0,
            contact: true,
        };

        let result = point.get_initial_step(Vector2Df::down());
        assert!(result.0 == Point::up(), "Position should be the same");
        assert!(result.1 == Vector2Df::zero(), "Velocity should be zero");
        assert!(
            result.2 == Vector2Df::up(),
            "Previous should copy last position"
        );
    }

    #[test]
    fn initial_step_air_friction() {
        let point = EntityPointSnapshot {
            position: Point::up(),
            velocity: Point::up(),
            previous_position: Point::down(),
            contact_friction: 0.0,
            air_friction: 0.5,
            contact: true,
        };

        let result = point.get_initial_step(Vector2Df::down());
        assert!(result.0 == Point::up(), "Position should be the same");
        assert!(result.1 == Vector2Df::zero(), "Velocity should be zero");
        assert!(
            result.2 == Vector2Df::up(),
            "Previous position should copy last position"
        );
    }
}
