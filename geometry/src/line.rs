use std::fmt::Debug;

use vector2d::Vector2Df;

use crate::Point;

#[derive(PartialEq, Debug)]
pub struct Line(Point, Point);

#[derive(PartialEq, Debug)]
enum Orientation {
    Collinear,
    Clockwise,
    Counterclockwise,
}

fn between(bound0: f64, value: f64, bound1: f64) -> bool {
    bound0.min(bound1) <= value && value <= bound0.max(bound1)
}

impl Line {
    pub fn new(point1: Point, point2: Point) -> Line {
        Line(point1, point2)
    }

    pub fn from_tuple(points: (Point, Point)) -> Line {
        Line(points.0, points.1)
    }

    pub fn p0(&self) -> Point {
        self.0
    }

    pub fn p1(&self) -> Point {
        self.1
    }

    pub fn get_vector(&self) -> Vector2Df {
        self.1 - self.0
    }

    pub fn intersects(&self, other: &Self) -> bool {
        let orientations = (
            Line::orientation((self.0, self.1, other.0)),
            Line::orientation((self.0, self.1, other.1)),
            Line::orientation((other.0, other.1, self.0)),
            Line::orientation((other.0, other.1, self.1)),
        );

        let collinear_checks = (
            orientations.0 == Orientation::Collinear
                && between(self.0.x, other.0.x, self.1.x)
                && between(self.0.y, other.0.y, self.1.y),
            orientations.1 == Orientation::Collinear
                && between(self.0.x, other.1.x, self.1.x)
                && between(self.0.y, other.1.y, self.1.y),
            orientations.2 == Orientation::Collinear
                && between(other.0.x, self.0.x, other.1.x)
                && between(other.0.y, self.0.y, other.1.y),
            orientations.3 == Orientation::Collinear
                && between(other.0.x, self.1.x, other.1.x)
                && between(other.0.y, self.1.y, other.1.y),
        );

        orientations.0 != orientations.1 && orientations.2 != orientations.3
            || collinear_checks.0
            || collinear_checks.1
            || collinear_checks.2
            || collinear_checks.3
    }

    fn orientation(points: (Point, Point, Point)) -> Orientation {
        let cross = (points.1.y - points.0.y) * (points.2.x - points.1.x)
            - (points.1.x - points.0.x) * (points.2.y - points.1.y);

        if cross == 0.0 {
            Orientation::Collinear
        } else if cross > 0.0 {
            Orientation::Clockwise
        } else {
            Orientation::Counterclockwise
        }
    }

    pub fn contains_point(&self, point: Point) -> bool {
        Vector2Df::distance(self.0, point) + Vector2Df::distance(self.1, point)
            == Vector2Df::distance(self.0, self.1)
    }

    /** Returns the closest point that lies on this line from another point */
    pub fn closest_included_point_from(&self, point: Point) -> Point {
        let delta = self.get_vector();

        if delta == Vector2Df::zero() {
            self.0
        } else {
            let alignment = Vector2Df::dot(point - self.0, delta);
            let percent_along_line = (alignment / delta.length_squared()).clamp(0.0, 1.0);
            self.0 + percent_along_line * delta
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Line, Point};

    #[test]
    fn equal() {
        let line1 = Line::new(Point::new(0.0, 0.0), Point::new(1.0, 1.0));
        let line2 = Line::new(Point::new(0.0, 0.0), Point::new(1.0, 1.0));
        let line3 = Line::new(Point::new(0.0, 0.0), Point::new(1.0, 2.0));
        let line4 = Line::new(Point::new(1.0, 0.0), Point::new(1.0, 1.0));
        assert!(line1 == line2, "same endpoints should equal");
        assert!(line1 != line3, "different second point should not equal");
        assert!(line1 != line4, "different first point should not equal");
    }

    #[test]
    fn intersection() {
        let line1 = Line::new(Point::new(-1.0, -1.0), Point::new(1.0, 1.0));
        let line2 = Line::new(Point::new(1.0, -1.0), Point::new(-1.0, 1.0));
        let line3 = Line::new(Point::new(1.0, -1.0), Point::new(0.0, 0.0));
        let line4 = Line::new(Point::new(0.0, 0.0), Point::new(0.0, 0.0));
        let line5 = Line::new(Point::new(-2.0, -2.0), Point::new(0.0, 0.0));
        assert!(line1.intersects(&line2), "lines should intersect");
        assert!(line1.intersects(&line3), "line endpoint should intersect");
        assert!(line1.intersects(&line4), "line point should intersect");
        assert!(line1.intersects(&line5), "collinear line should intersect");
    }

    #[test]
    fn contains_point() {
        let line = Line::new(Point::new(-1.0, -1.0), Point::new(1.0, 1.0));
        let point1 = Point::new(0.0, 0.0);
        let point2 = Point::new(-1.0, -1.0);
        let point3 = Point::new(-2.0, -2.0);
        let point4 = Point::new(1.0, -1.0);
        assert!(line.contains_point(point1), "line should contain point");
        assert!(line.contains_point(point2), "line should contain endpoint");
        assert!(
            !line.contains_point(point3),
            "line should not contain point extending segment"
        );
        assert!(
            !line.contains_point(point4),
            "line should not contain point away from segment"
        );
    }

    #[test]
    fn closet_point_on_line() {
        let line = Line::new(Point::new(-2.0, -1.0), Point::new(2.0, 1.0));
        let point1 = Point::new(-1.0, 2.0);
        let point2 = Point::new(-3.0, -3.0);
        let point3 = Point::new(0.0, 0.0);
        assert!(
            line.closest_included_point_from(point1) == Point::new(0.0, 0.0),
            "closest point should form line with other point perpendicular to this line"
        );
        assert!(
            line.closest_included_point_from(point2) == Point::new(-2.0, -1.0),
            "closest point away from line should be line endpoint"
        );
        assert!(
            line.closest_included_point_from(point3) == Point::new(0.0, 0.0),
            "closest point already on line should be the same point"
        );
    }
}
