use geometry::Point;
use vector2d::Vector2Df;

pub trait EntityPointLogic {
    fn position(&self) -> Point;
    fn velocity(&self) -> Vector2Df;
    fn previous_position(&self) -> Point;
    fn air_friction(&self) -> f64;
    fn contact_friction(&self) -> f64;
    fn is_contact(&self) -> bool;
    fn update(
        &mut self,
        new_position: Point,
        new_velocity: Vector2Df,
        new_previous_position: Vector2Df,
    );

    fn process_initial_step(&mut self, gravity: Vector2Df) {
        let computed_velocity = self.position() - self.previous_position();
        let new_velocity = computed_velocity * (1.0 - self.air_friction()) + gravity;
        let new_position = self.position() + new_velocity;
        self.update(new_position, new_velocity, self.position());
    }
}

#[cfg(test)]
mod tests {
    use geometry::Point;
    use vector2d::Vector2Df;

    use crate::entity::logic::point::EntityPointLogic;

    struct PurePoint(Point, Vector2Df, Point, f64, f64, bool);

    impl EntityPointLogic for PurePoint {
        fn position(&self) -> Point {
            self.0
        }
        fn velocity(&self) -> Vector2Df {
            self.1
        }
        fn previous_position(&self) -> Point {
            self.2
        }
        fn air_friction(&self) -> f64 {
            self.3
        }
        fn contact_friction(&self) -> f64 {
            self.4
        }
        fn is_contact(&self) -> bool {
            self.5
        }
        fn update(
            &mut self,
            new_position: Point,
            new_velocity: Vector2Df,
            new_previous_position: Vector2Df,
        ) {
            self.0 = new_position;
            self.1 = new_velocity;
            self.2 = new_previous_position;
        }
    }

    #[test]
    fn initial_step() {
        let mut point = PurePoint(
            Point::zero(),
            Vector2Df::zero(),
            Vector2Df::zero(),
            0.0,
            0.0,
            true,
        );
        point.process_initial_step(Vector2Df::zero());
        assert!(point.0 == Point::zero(), "Position should be zero");
        assert!(point.1 == Vector2Df::zero(), "Velocity should be zero");
        assert!(
            point.2 == Vector2Df::zero(),
            "Previous position should be zero"
        );
        let mut point = PurePoint(Point::up(), Point::up(), Point::zero(), 0.0, 0.0, true);
        point.process_initial_step(Vector2Df::zero());
        assert!(point.0 == Point::up() * 2.0, "Position should increase");
        assert!(point.1 == Vector2Df::up(), "Velocity should stay the same");
        assert!(
            point.2 == Vector2Df::up(),
            "Previous position should be the last position"
        );
        let mut point = PurePoint(Point::up(), Point::up(), Point::zero(), 0.0, 0.0, true);
        point.process_initial_step(Vector2Df::up() * 0.5);
        assert!(point.0 == Point::zero(), "Position should be zero");
        assert!(point.1 == Vector2Df::zero(), "Velocity should be zero");
        assert!(
            point.2 == Vector2Df::zero(),
            "Previous position should be zero"
        );
        let mut point = PurePoint(Point::up(), Point::up(), Point::zero(), 0.5, 0.0, true);
        point.process_initial_step(Vector2Df::zero());
        assert!(point.0 == Point::zero(), "Position should be zero");
        assert!(point.1 == Vector2Df::zero(), "Velocity should be zero");
        assert!(
            point.2 == Vector2Df::zero(),
            "Previous position should be zero"
        );
    }
}
