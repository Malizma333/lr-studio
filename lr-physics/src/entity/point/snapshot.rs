use geometry::Point;
use vector2d::Vector2Df;

pub(crate) struct EntityPointSnapshot {
    contact: bool,
    contact_friction: f64,
    air_friction: f64,
    position: Point,
    velocity: Vector2Df,
    previous_position: Point,
}

impl EntityPointSnapshot {
    pub(super) fn new(
        position: Point,
        velocity: Vector2Df,
        previous_position: Point,
        contact_friction: f64,
        air_friction: f64,
        contact: bool,
    ) -> Self {
        EntityPointSnapshot {
            contact,
            contact_friction,
            air_friction,
            position,
            velocity,
            previous_position,
        }
    }

    pub fn is_contact(&self) -> bool {
        self.contact
    }

    pub fn position(&self) -> Point {
        self.position
    }

    fn process_initial_step(&mut self, gravity: Vector2Df) {
        let computed_velocity = self.position() - self.previous_position;
        let new_velocity = computed_velocity * (1.0 - self.air_friction) + gravity;
        let new_position = self.position() + new_velocity;
        self.update(new_position, new_velocity, self.position);
    }
}

#[cfg(test)]
mod tests {
    use geometry::Point;
    use vector2d::Vector2Df;

    use crate::entity::point::snapshot::EntityPointSnapshot;

    #[test]
    fn initial_step() {
        let mut point = EntityPointSnapshot::new(
            Point::zero(),
            Vector2Df::zero(),
            Vector2Df::zero(),
            0.0,
            0.0,
            true,
        );
        point.process_initial_step(Vector2Df::zero());
        assert!(point.position == Point::zero(), "Position should be zero");
        assert!(
            point.velocity == Vector2Df::zero(),
            "Velocity should be zero"
        );
        assert!(
            point.previous_position == Vector2Df::zero(),
            "Previous position should be zero"
        );
        let mut point =
            EntityPointSnapshot::new(Point::up(), Point::up(), Point::zero(), 0.0, 0.0, true);
        point.process_initial_step(Vector2Df::zero());
        assert!(
            point.position == Point::up() * 2.0,
            "Position should increase"
        );
        assert!(
            point.velocity == Vector2Df::up(),
            "Velocity should stay the same"
        );
        assert!(
            point.previous_position == Vector2Df::up(),
            "Previous position should be the last position"
        );
        let mut point =
            EntityPointSnapshot::new(Point::up(), Point::up(), Point::zero(), 0.0, 0.0, true);
        point.process_initial_step(Vector2Df::up() * 0.5);
        assert!(point.position == Point::zero(), "Position should be zero");
        assert!(
            point.velocity == Vector2Df::zero(),
            "Velocity should be zero"
        );
        assert!(
            point.previous_position == Vector2Df::zero(),
            "Previous position should be zero"
        );
        let mut point =
            EntityPointSnapshot::new(Point::up(), Point::up(), Point::zero(), 0.5, 0.0, true);
        point.process_initial_step(Vector2Df::zero());
        assert!(point.position == Point::zero(), "Position should be zero");
        assert!(
            point.velocity == Vector2Df::zero(),
            "Velocity should be zero"
        );
        assert!(
            point.previous_position == Vector2Df::zero(),
            "Previous position should be zero"
        );
    }
}
