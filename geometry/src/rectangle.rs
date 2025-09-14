use crate::{Line, Point, between};
use vector2d::Vector2Df;

#[derive(Debug)]
pub struct Rectangle {
    origin: Point,
    size: Vector2Df,
}

impl Rectangle {
    pub fn new(corner0: Point, corner1: Point) -> Rectangle {
        let min_x = corner0.x().min(corner1.x());
        let max_x = corner0.x().max(corner1.x());
        let min_y = corner0.y().min(corner1.y());
        let max_y = corner0.y().max(corner1.y());
        Rectangle {
            origin: Point::new(min_x, min_y),
            size: Vector2Df::new(max_x - min_x, max_y - min_y),
        }
    }

    pub fn origin(&self) -> Point {
        self.origin
    }

    pub fn size(&self) -> Vector2Df {
        self.size
    }

    /** Returns the line segment that represents the bottom side of this rectangle */
    pub fn bottom(&self) -> Line {
        Line(
            Point::new(self.origin.x(), self.origin.y()),
            Point::new(self.origin.x() + self.size.x(), self.origin.y()),
        )
    }

    /** Returns the line segment that represents the top side of this rectangle */
    pub fn top(&self) -> Line {
        Line(
            Point::new(self.origin.x(), self.origin.y() + self.size.y()),
            Point::new(
                self.origin.x() + self.size.x(),
                self.origin.y() + self.size.y(),
            ),
        )
    }

    /** Returns the line segment that represents the left side of this rectangle */
    pub fn left(&self) -> Line {
        Line(
            Point::new(self.origin.x(), self.origin.y()),
            Point::new(self.origin.x(), self.origin.y() + self.size.y()),
        )
    }

    /** Returns the line segment that represents the right side of this rectangle */
    pub fn right(&self) -> Line {
        Line(
            Point::new(self.origin.x() + self.size.x(), self.origin.y()),
            Point::new(
                self.origin.x() + self.size.x(),
                self.origin.y() + self.size.y(),
            ),
        )
    }

    /** Returns the point at the bottom left of this rectangle */
    pub fn bottom_left(&self) -> Point {
        Point::new(self.origin.x(), self.origin.y())
    }

    /** Returns the point at the bottom right of this rectangle */
    pub fn bottom_right(&self) -> Point {
        Point::new(self.origin.x() + self.size.x(), self.origin.y())
    }

    /** Returns the point at the top left of this rectangle */
    pub fn top_left(&self) -> Point {
        Point::new(self.origin.x(), self.origin.y() + self.size.y())
    }

    /** Returns the point at the top right of this rectangle */
    pub fn top_right(&self) -> Point {
        Point::new(
            self.origin.x() + self.size.x(),
            self.origin.y() + self.size.y(),
        )
    }

    /** Whether this rectangle contains a segment of a line, including lines with endpoints outside of the rectangle */
    pub fn contains(&self, line: &Line) -> bool {
        let first_point_inside =
            between(self.origin.x(), line.0.x(), self.origin.x() + self.size.x())
                && between(self.origin.y(), line.0.y(), self.origin.y() + self.size.y());
        let second_point_inside =
            between(self.origin.x(), line.1.x(), self.origin.x() + self.size.x())
                && between(self.origin.y(), line.1.y(), self.origin.y() + self.size.y());

        (first_point_inside && second_point_inside)
            || self.bottom().intersects(line)
            || self.top().intersects(line)
            || self.left().intersects(line)
            || self.right().intersects(line)
    }
}

#[cfg(test)]
mod tests {
    use vector2d::Vector2Df;

    use crate::{Line, Point, Rectangle};

    #[test]
    fn getters() {
        let rect = Rectangle::new(Point::new(10.0, 5.0), Point::new(-3.0, 6.0));
        assert!(
            rect.origin() == Point::new(-3.0, 5.0),
            "origin should be minimum point"
        );
        assert!(
            rect.size() == Vector2Df::new(13.0, 1.0),
            "size should be width and height components"
        );
    }

    #[test]
    fn bounds() {
        let rect = Rectangle::new(Point::new(10.0, 5.0), Point::new(-3.0, 6.0));
        assert!(
            rect.bottom() == Line(Point::new(-3.0, 5.0), Point::new(10.0, 5.0)),
            "bottom should be lowest y points"
        );
        assert!(
            rect.top() == Line(Point::new(-3.0, 6.0), Point::new(10.0, 6.0)),
            "top should be highest y points"
        );
        assert!(
            rect.left() == Line(Point::new(-3.0, 5.0), Point::new(-3.0, 6.0)),
            "left should be lowest x points"
        );
        assert!(
            rect.right() == Line(Point::new(10.0, 5.0), Point::new(10.0, 6.0)),
            "right should be highest x points"
        );
    }

    #[test]
    fn contains() {
        let rect = Rectangle::new(Point::new(-1.0, -3.0), Point::new(1.0, 3.0));
        let line1 = Line(Point::new(0.0, -2.0), Point::new(0.0, 2.0));
        let line2 = Line(Point::new(0.0, -2.0), Point::new(-2.0, 4.0));
        let line3 = Line(Point::new(2.0, -5.0), Point::new(-2.0, 4.0));
        let line4 = Line(Point::new(-2.0, -5.0), Point::new(-2.0, 4.0));
        assert!(
            rect.contains(&line1),
            "rectangle should contain line inside"
        );
        assert!(
            rect.contains(&line2),
            "rectangle should contain line with one point inside"
        );
        assert!(
            rect.contains(&line3),
            "rectangle should contain line intersecting it"
        );
        assert!(
            !rect.contains(&line4),
            "rectangle should not contain line outside"
        );
    }

    #[test]
    fn contains_edges() {
        let rect = Rectangle::new(Point::new(-1.0, -3.0), Point::new(1.0, 3.0));
        let line1 = Line(Point::new(-2.0, -3.0), Point::new(3.0, -3.0));
        let line2 = Line(Point::new(-2.0, 3.0), Point::new(4.0, 3.0));
        let line3 = Line(Point::new(-1.0, -5.0), Point::new(-1.0, 4.0));
        let line4 = Line(Point::new(1.0, -3.0), Point::new(1.0, 4.0));
        let line5 = Line(Point::new(-2.0, -2.0), Point::new(0.0, -4.0));
        assert!(
            rect.contains(&line1),
            "rectangle should contain line collinear with bottom"
        );
        assert!(
            rect.contains(&line2),
            "rectangle should contain line collinear with top"
        );
        assert!(
            rect.contains(&line3),
            "rectangle should contain line collinear with left"
        );
        assert!(
            rect.contains(&line4),
            "rectangle should contain line collinear with right"
        );
        assert!(
            rect.contains(&line5),
            "rectangle should contain line intersecting corner"
        );
    }
}
