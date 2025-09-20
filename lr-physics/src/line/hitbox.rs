use vector2d::Vector2Df;

use crate::{entity::ContactPoint, line::computed::ComputedLineProperties};

pub(crate) const HITBOX_HEIGHT: f64 = 10.0;

pub trait Hitbox: ComputedLineProperties {
    fn interact(
        &self,
        point: &mut ContactPoint,
        distance_from_line_top: f64,
        position_between_ends: f64,
    );

    fn check_interaction(&self, point: &mut ContactPoint) -> bool {
        let offset_from_point = point.position() - self.endpoints().0;
        let moving_into_line = Vector2Df::dot(self.normal_unit(), point.velocity()) > 0.0;
        let distance_from_line_top = Vector2Df::dot(self.normal_unit(), offset_from_point);
        let position_between_ends =
            Vector2Df::dot(self.vector(), offset_from_point) * self.inverse_length_squared();

        if moving_into_line
            && 0.0 < distance_from_line_top
            && distance_from_line_top < HITBOX_HEIGHT
            && self.left_limit() <= position_between_ends
            && position_between_ends <= self.right_limit()
        {
            self.interact(point, distance_from_line_top, position_between_ends);
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use geometry::Point;
    use vector2d::Vector2Df;

    use crate::{
        entity::ContactPoint,
        line::{computed::ComputedLineProperties, hitbox::Hitbox},
    };
    struct SimpleStruct(pub Vector2Df, pub Vector2Df, pub bool, pub bool, pub bool);

    impl ComputedLineProperties for SimpleStruct {
        fn endpoints(&self) -> (geometry::Point, geometry::Point) {
            (self.0, self.1)
        }

        fn flipped(&self) -> bool {
            self.2
        }

        fn extended_left(&self) -> bool {
            self.3
        }

        fn extended_right(&self) -> bool {
            self.4
        }
    }

    impl Hitbox for SimpleStruct {
        fn interact(
            &self,
            point: &mut crate::entity::ContactPoint,
            distance_from_line_top: f64,
            position_between_ends: f64,
        ) {
        }
    }

    #[test]
    fn interaction() {
        let line = SimpleStruct(
            Point::new(-10.0, 0.0),
            Point::new(10.0, 0.0),
            false,
            false,
            false,
        );
        let mut contact_point = ContactPoint::new(Point::zero(), 0.0);
        contact_point.update(Point::one(), Vector2Df::one(), Point::zero());
        assert!(
            line.check_interaction(&mut contact_point),
            "Contact point moving into line within hitbox should interact"
        );
    }

    #[test]
    fn point_moving_out_of_line() {
        let line = SimpleStruct(
            Point::new(-10.0, 0.0),
            Point::new(10.0, 0.0),
            false,
            false,
            false,
        );
        let mut contact_point = ContactPoint::new(Point::zero(), 0.0);
        contact_point.update(Point::one(), -1.0 * Vector2Df::one(), Point::zero());
        assert!(
            !line.check_interaction(&mut contact_point),
            "Contact point moving out of line should not interact"
        );
    }

    #[test]
    fn point_above_line() {
        let line = SimpleStruct(
            Point::new(-10.0, 0.0),
            Point::new(10.0, 0.0),
            false,
            false,
            false,
        );
        let mut contact_point = ContactPoint::new(Point::zero(), 0.0);
        contact_point.update(Point::new(0.0, -1.0), Vector2Df::one(), Point::zero());
        assert!(
            !line.check_interaction(&mut contact_point),
            "Contact point moving above line should not interact"
        );
    }

    #[test]
    fn point_above_flipped_line() {
        let line = SimpleStruct(
            Point::new(-10.0, 0.0),
            Point::new(10.0, 0.0),
            true,
            false,
            false,
        );
        let mut contact_point = ContactPoint::new(Point::zero(), 0.0);
        contact_point.update(
            Point::new(0.0, -1.0),
            -1.0 * Vector2Df::one(),
            Point::zero(),
        );
        assert!(
            line.check_interaction(&mut contact_point),
            "Contact point moving above flipped line should interact"
        );
    }

    #[test]
    fn point_below_line() {
        let line = SimpleStruct(
            Point::new(-10.0, 0.0),
            Point::new(10.0, 0.0),
            false,
            false,
            false,
        );
        let mut contact_point = ContactPoint::new(Point::zero(), 0.0);
        contact_point.update(Point::new(0.0, 12.0), Vector2Df::one(), Point::zero());
        assert!(
            !line.check_interaction(&mut contact_point),
            "Contact point moving below line should not interact"
        );
    }

    #[test]
    fn point_to_left_of_line() {
        let line = SimpleStruct(
            Point::new(-10.0, 0.0),
            Point::new(10.0, 0.0),
            false,
            false,
            false,
        );
        let mut contact_point = ContactPoint::new(Point::zero(), 0.0);
        contact_point.update(Point::new(-11.0, 5.0), Vector2Df::one(), Point::zero());
        assert!(
            !line.check_interaction(&mut contact_point),
            "Contact point moving left of line should not interact"
        );
    }

    #[test]
    fn point_to_right_of_line() {
        let line = SimpleStruct(
            Point::new(-10.0, 0.0),
            Point::new(10.0, 0.0),
            false,
            false,
            false,
        );
        let mut contact_point = ContactPoint::new(Point::zero(), 0.0);
        contact_point.update(Point::new(11.0, 5.0), Vector2Df::one(), Point::zero());
        assert!(
            !line.check_interaction(&mut contact_point),
            "Contact point moving right of line should not interact"
        );
    }

    #[test]
    fn point_to_left_of_line_extended() {
        let line = SimpleStruct(
            Point::new(-10.0, 0.0),
            Point::new(10.0, 0.0),
            false,
            true,
            false,
        );
        let mut contact_point = ContactPoint::new(Point::zero(), 0.0);
        contact_point.update(Point::new(-11.0, 5.0), Vector2Df::one(), Point::zero());
        assert!(
            line.check_interaction(&mut contact_point),
            "Contact point moving left of line with extension should interact"
        );
    }

    #[test]
    fn point_to_right_of_line_extended() {
        let line = SimpleStruct(
            Point::new(-10.0, 0.0),
            Point::new(10.0, 0.0),
            false,
            false,
            true,
        );
        let mut contact_point = ContactPoint::new(Point::zero(), 0.0);
        contact_point.update(Point::new(11.0, 5.0), Vector2Df::one(), Point::zero());
        assert!(
            line.check_interaction(&mut contact_point),
            "Contact point moving right of line with extension should interact"
        );
    }
}

// Put line-specific implementations somewhere else?
//             let new_position = point.position() - (self.normal_unit() * distance_from_line_top);
//
//             let friction_vector =
//                 (self.normal_unit().rotate_cw() * point.friction) * distance_from_line_top;
//
//             if point.previous_position().x() >= new_position.x() {
//                 friction_vector.x *= -1;
//             }
//
//             if point.previous_position().y() < new_position.y() {
//                 friction_vector.y *= -1;
//             }
//
//             let new_previous_position =
//                 point.base.previous_position + friction_vector - self.acceleration_vector;
//
//             (new_position, new_previous_position)
