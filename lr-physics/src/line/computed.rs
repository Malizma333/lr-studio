use geometry::Point;
use vector2d::Vector2Df;

use crate::line::hitbox::HITBOX_HEIGHT;

pub trait ComputedLineProperties {
    fn endpoints(&self) -> (Point, Point);
    fn flipped(&self) -> bool;
    fn extended_left(&self) -> bool;
    fn extended_right(&self) -> bool;

    fn vector(&self) -> Vector2Df {
        self.endpoints().1 - self.endpoints().0
    }

    fn length(&self) -> f64 {
        self.vector().length()
    }

    fn inverse_length_squared(&self) -> f64 {
        1.0 / self.vector().length_squared()
    }

    fn unit(&self) -> Vector2Df {
        self.vector() * (1.0 / self.length())
    }

    fn normal_unit(&self) -> Vector2Df {
        if self.flipped() {
            self.unit().rotate_cw()
        } else {
            self.unit().rotate_ccw()
        }
    }

    fn extension_ratio(&self) -> f64 {
        const MAX_EXTENSION_SIZE: f64 = 0.25;
        MAX_EXTENSION_SIZE.min(HITBOX_HEIGHT / self.length())
    }

    fn left_limit(&self) -> f64 {
        if self.extended_left() {
            -self.extension_ratio()
        } else {
            0.0
        }
    }

    fn right_limit(&self) -> f64 {
        if self.extended_right() {
            1.0 + self.extension_ratio()
        } else {
            1.0
        }
    }
}

#[cfg(test)]
mod tests {
    use geometry::Point;
    use vector2d::Vector2Df;

    use crate::line::computed::ComputedLineProperties;

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

    #[test]
    fn vector() {
        let x = SimpleStruct(
            Point::new(1.0, 2.0),
            Point::new(4.0, 6.0),
            false,
            false,
            false,
        );
        assert!(
            x.vector() == Vector2Df::new(3.0, 4.0),
            "vector should be difference of endpoints"
        );
    }

    #[test]
    fn vector_length() {
        let x = SimpleStruct(
            Point::new(1.0, 2.0),
            Point::new(4.0, 6.0),
            false,
            false,
            false,
        );
        assert!(x.length() == 5.0, "length should be length of vector");
    }

    #[test]
    fn vector_inverse_length_squared() {
        let x = SimpleStruct(
            Point::new(1.0, 2.0),
            Point::new(4.0, 6.0),
            false,
            false,
            false,
        );
        assert!(
            x.inverse_length_squared() == 1.0 / 25.0,
            "inverse length squared should be correct"
        );
    }

    #[test]
    fn unit_vector() {
        let x = SimpleStruct(
            Point::new(1.0, 2.0),
            Point::new(6.0, 14.0),
            false,
            false,
            false,
        );
        assert!(
            x.unit() == (1.0 / 13.0) * Vector2Df::new(5.0, 12.0),
            "unit vector should be correct"
        );
    }

    #[test]
    fn normal_vector() {
        let x = SimpleStruct(
            Point::new(1.0, 2.0),
            Point::new(6.0, 14.0),
            false,
            false,
            false,
        );
        assert!(
            x.normal_unit() == (1.0 / 13.0) * Vector2Df::new(-12.0, 5.0),
            "normal vector should rotate counterclockwise if not flipped"
        );
        let x = SimpleStruct(
            Point::new(1.0, 2.0),
            Point::new(6.0, 14.0),
            true,
            false,
            false,
        );
        assert!(
            x.normal_unit() == (1.0 / 13.0) * Vector2Df::new(12.0, -5.0),
            "normal vector should rotate clockwise if flipped"
        );
    }

    #[test]
    fn unit_overflow() {
        let x = SimpleStruct(
            Point::new(4.0, 3.0),
            Point::new(8.0, 6.0),
            false,
            false,
            false,
        );
        assert!(
            x.unit() == (1.0 / 5.0) * Vector2Df::new(4.0, 3.0),
            "unit should multiply by inverse of length"
        );
    }

    #[test]
    fn extension_ratio() {
        let x = SimpleStruct(
            Point::new(0.0, 0.0),
            Point::new(400.0, 300.0),
            false,
            false,
            false,
        );
        assert!(
            x.extension_ratio() == 0.02,
            "Extension ratio should be correct"
        );
        let x = SimpleStruct(
            Point::new(0.0, 0.0),
            Point::new(12.0, 9.0),
            false,
            false,
            false,
        );
        assert!(
            x.extension_ratio() == 0.25,
            "Extension ratio should cap at 0.25"
        );
    }

    #[test]
    fn limits() {
        let x = SimpleStruct(
            Point::new(0.0, 0.0),
            Point::new(1.0, 1.0),
            false,
            false,
            false,
        );
        assert!(x.left_limit() == 0.0, "Left limit should not be extended");
        assert!(x.right_limit() == 1.0, "Right limit should not be extended");
        let x = SimpleStruct(
            Point::new(0.0, 0.0),
            Point::new(1.0, 1.0),
            false,
            true,
            true,
        );
        assert!(x.left_limit() == -0.25, "Left limit should extend");
        assert!(x.right_limit() == 1.25, "Right limit should extend");
    }
}
