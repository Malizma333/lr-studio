use vector2d::Vector2Df;

use crate::{Line, Point};

#[derive(Debug)]
pub struct Circle {
    center: Point,
    radius: f64,
}

impl Circle {
    pub fn new(center: Point, radius: f64) -> Circle {
        Circle { center, radius }
    }

    pub fn center(&self) -> Point {
        self.center
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }

    pub fn contains_point(&self, point: Point) -> bool {
        Vector2Df::distance_squared(point, self.center()) <= self.radius * self.radius
    }

    /** Whether this circle includes part of a line, including lines with endpoints outside of the circle that intersect it */
    pub fn includes_part_of_line(&self, line: &Line) -> bool {
        self.contains_point(line.closest_included_point_from(self.center))
    }
}

#[cfg(test)]
mod tests {
    use crate::{Circle, Line, Point};

    #[test]
    fn line_inclusion() {
        let circle = Circle::new(Point::new(0.0, 0.0), 3.0);
        let line1 = Line::new(Point::new(-1.0, -1.0), Point::new(0.0, 1.0));
        let line2 = Line::new(Point::new(-5.0, -6.0), Point::new(-2.0, 0.0));
        let line3 = Line::new(Point::new(-3.0, -5.0), Point::new(3.0, 5.0));
        let line4 = Line::new(Point::new(-6.0, 2.0), Point::new(-5.0, 3.0));
        assert!(
            circle.includes_part_of_line(&line1),
            "circle should contain line inside"
        );
        assert!(
            circle.includes_part_of_line(&line2),
            "circle should contain line with one point inside"
        );
        assert!(
            circle.includes_part_of_line(&line3),
            "circle should contain line intersecting it"
        );
        assert!(
            !circle.includes_part_of_line(&line4),
            "circle should not contain line outside"
        );
    }

    #[test]
    fn line_inclusion_edge_cases() {
        let circle = Circle::new(Point::new(0.0, 0.0), 3.0);
        let line1 = Line::new(Point::new(-3.0, -3.0), Point::new(3.0, -3.0));
        let line2 = Line::new(Point::new(-3.0, 0.0), Point::new(-4.0, -5.0));
        assert!(
            circle.includes_part_of_line(&line1),
            "circle should contain line tangent to circumference"
        );
        assert!(
            circle.includes_part_of_line(&line2),
            "circle should contain line with point touching circumference"
        );
    }
}
