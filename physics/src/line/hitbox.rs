use geometry::Point;
use vector2d::Vector2Df;

use crate::{entity::point::snapshot::EntityPointSnapshot, line::computed::ComputedLineProperties};

pub(crate) const HITBOX_HEIGHT: f64 = 10.0;

pub trait Hitbox: ComputedLineProperties {
    /** Returns the new (position, previous position) to update a point with after it interacts with this line\
    (The previous position is not necessarily `position - velocity`, it represents how much force is applied
    on the momentum tick due to forces such as friction)
    */
    fn interact(
        &self,
        point: &EntityPointSnapshot,
        distance_from_line_top: f64,
        position_between_ends: f64,
    ) -> Option<(Point, Point)>;

    fn check_interaction(&self, point: &EntityPointSnapshot) -> Option<(Point, Point)> {
        if !point.is_contact() {
            return None;
        }

        let offset_from_point = point.position() - self.properties().endpoints().0;
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
            self.interact(point, distance_from_line_top, position_between_ends)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use geometry::Point;
    use vector2d::Vector2Df;

    use crate::{
        entity::point::snapshot::EntityPointSnapshot,
        line::{
            computed::{ComputedLineProperties, ComputedProperties},
            hitbox::Hitbox,
        },
    };
    struct SimpleStruct(pub Point, pub Point, pub bool, pub bool, pub bool);

    impl ComputedLineProperties for SimpleStruct {
        fn properties(&self) -> ComputedProperties {
            ComputedProperties::new((self.0, self.1), self.2, self.3, self.4)
        }
    }

    impl Hitbox for SimpleStruct {
        fn interact(
            &self,
            _point: &EntityPointSnapshot,
            _distance_from_line_top: f64,
            _position_between_ends: f64,
        ) -> Option<(Point, Point)> {
            Some((Point::zero(), Point::zero()))
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
        let contact_point = EntityPointSnapshot::new(
            Point::one(),
            Vector2Df::one(),
            Point::zero(),
            0.0,
            0.0,
            true,
        );
        assert!(
            line.check_interaction(&contact_point).is_some(),
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
        let contact_point = EntityPointSnapshot::new(
            Point::one(),
            -1.0 * Vector2Df::one(),
            Point::zero(),
            0.0,
            0.0,
            true,
        );
        assert!(
            line.check_interaction(&contact_point).is_none(),
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
        let contact_point = EntityPointSnapshot::new(
            Point::new(0.0, -1.0),
            Vector2Df::one(),
            Point::zero(),
            0.0,
            0.0,
            true,
        );
        assert!(
            line.check_interaction(&contact_point).is_none(),
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
        let contact_point = EntityPointSnapshot::new(
            Point::new(0.0, -1.0),
            -1.0 * Vector2Df::one(),
            Point::zero(),
            0.0,
            0.0,
            true,
        );
        assert!(
            line.check_interaction(&contact_point).is_some(),
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
        let contact_point = EntityPointSnapshot::new(
            Point::new(0.0, 12.0),
            Vector2Df::one(),
            Point::zero(),
            0.0,
            0.0,
            true,
        );
        assert!(
            line.check_interaction(&contact_point).is_none(),
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
        let contact_point = EntityPointSnapshot::new(
            Point::new(-11.0, 5.0),
            Vector2Df::one(),
            Point::zero(),
            0.0,
            0.0,
            true,
        );
        assert!(
            line.check_interaction(&contact_point).is_none(),
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
        let contact_point = EntityPointSnapshot::new(
            Point::new(11.0, 5.0),
            Vector2Df::one(),
            Point::zero(),
            0.0,
            0.0,
            true,
        );
        assert!(
            line.check_interaction(&contact_point).is_none(),
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
        let contact_point = EntityPointSnapshot::new(
            Point::new(-11.0, 5.0),
            Vector2Df::one(),
            Point::zero(),
            0.0,
            0.0,
            true,
        );
        assert!(
            line.check_interaction(&contact_point).is_some(),
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
        let contact_point = EntityPointSnapshot::new(
            Point::new(11.0, 5.0),
            Vector2Df::one(),
            Point::zero(),
            0.0,
            0.0,
            true,
        );
        assert!(
            line.check_interaction(&contact_point).is_some(),
            "Contact point moving right of line with extension should interact"
        );
    }
}
