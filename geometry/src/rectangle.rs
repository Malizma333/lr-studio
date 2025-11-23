use crate::{Line, Point};
use vector2d::Vector2Df;

#[derive(Debug)]
pub struct Rectangle {
    origin: Point,
    size: Vector2Df,
}

impl Rectangle {
    pub fn new(corner0: Point, corner1: Point) -> Rectangle {
        let min_x = corner0.x.min(corner1.x);
        let max_x = corner0.x.max(corner1.x);
        let min_y = corner0.y.min(corner1.y);
        let max_y = corner0.y.max(corner1.y);
        Rectangle {
            origin: Point::new(min_x, min_y),
            size: Vector2Df::new(max_x - min_x, max_y - min_y),
        }
    }

    pub fn origin(&self) -> &Point {
        &self.origin
    }

    pub fn size(&self) -> &Vector2Df {
        &self.size
    }

    /** Returns the point at the bottom left of this rectangle */
    pub fn bottom_left(&self) -> Point {
        Point::new(self.origin.x, self.origin.y)
    }

    /** Returns the point at the bottom right of this rectangle */
    pub fn bottom_right(&self) -> Point {
        Point::new(self.origin.x + self.size.x, self.origin.y)
    }

    /** Returns the point at the top left of this rectangle */
    pub fn top_left(&self) -> Point {
        Point::new(self.origin.x, self.origin.y + self.size.y)
    }

    /** Returns the point at the top right of this rectangle */
    pub fn top_right(&self) -> Point {
        Point::new(self.origin.x + self.size.x, self.origin.y + self.size.y)
    }

    /** Returns the line segment that represents the bottom side of this rectangle */
    pub fn bottom(&self) -> Line {
        Line::new(self.bottom_left(), self.bottom_right())
    }

    /** Returns the line segment that represents the top side of this rectangle */
    pub fn top(&self) -> Line {
        Line::new(self.top_left(), self.top_right())
    }

    /** Returns the line segment that represents the left side of this rectangle */
    pub fn left(&self) -> Line {
        Line::new(self.bottom_left(), self.top_left())
    }

    /** Returns the line segment that represents the right side of this rectangle */
    pub fn right(&self) -> Line {
        Line::new(self.bottom_right(), self.top_right())
    }

    pub fn contains_point(&self, point: Point) -> bool {
        self.origin.x <= point.x
            && point.x <= self.origin.x + self.size.x
            && self.origin.y <= point.y
            && point.y <= self.origin.y + self.size.y
    }

    /** Whether this rectangle includes part of a line, including lines with endpoints outside of the rectangle that intersect it */
    pub fn includes_portion_of_line(&self, line: &Line) -> bool {
        self.contains_point(line.p0())
            || self.contains_point(line.p1())
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
            *rect.origin() == Point::new(-3.0, 5.0),
            "origin should be minimum point"
        );
        assert!(
            *rect.size() == Vector2Df::new(13.0, 1.0),
            "size should be width and height components"
        );
    }

    #[test]
    fn bounds() {
        let rect = Rectangle::new(Point::new(10.0, 5.0), Point::new(-3.0, 6.0));
        assert!(
            rect.bottom() == Line::new(Point::new(-3.0, 5.0), Point::new(10.0, 5.0)),
            "bottom should be lowest y points"
        );
        assert!(
            rect.top() == Line::new(Point::new(-3.0, 6.0), Point::new(10.0, 6.0)),
            "top should be highest y points"
        );
        assert!(
            rect.left() == Line::new(Point::new(-3.0, 5.0), Point::new(-3.0, 6.0)),
            "left should be lowest x points"
        );
        assert!(
            rect.right() == Line::new(Point::new(10.0, 5.0), Point::new(10.0, 6.0)),
            "right should be highest x points"
        );
    }

    #[test]
    fn line_inclusion() {
        let rect = Rectangle::new(Point::new(-1.0, -3.0), Point::new(1.0, 3.0));
        let line1 = Line::new(Point::new(0.0, -2.0), Point::new(0.0, 2.0));
        let line2 = Line::new(Point::new(0.0, -2.0), Point::new(-2.0, 4.0));
        let line3 = Line::new(Point::new(2.0, -5.0), Point::new(-2.0, 4.0));
        let line4 = Line::new(Point::new(-2.0, -5.0), Point::new(-2.0, 4.0));
        assert!(
            rect.includes_portion_of_line(&line1),
            "rectangle should contain line inside"
        );
        assert!(
            rect.includes_portion_of_line(&line2),
            "rectangle should contain line with one point inside"
        );
        assert!(
            rect.includes_portion_of_line(&line3),
            "rectangle should contain line intersecting it"
        );
        assert!(
            !rect.includes_portion_of_line(&line4),
            "rectangle should not contain line outside"
        );
    }

    #[test]
    fn line_inclusion_edge_cases() {
        let rect = Rectangle::new(Point::new(-1.0, -3.0), Point::new(1.0, 3.0));
        let line1 = Line::new(Point::new(-2.0, -3.0), Point::new(3.0, -3.0));
        let line2 = Line::new(Point::new(-2.0, 3.0), Point::new(4.0, 3.0));
        let line3 = Line::new(Point::new(-1.0, -5.0), Point::new(-1.0, 4.0));
        let line4 = Line::new(Point::new(1.0, -3.0), Point::new(1.0, 4.0));
        let line5 = Line::new(Point::new(-2.0, -2.0), Point::new(0.0, -4.0));
        let line6 = Line::new(Point::new(-5.0, -5.0), Point::new(-1.0, -1.0));
        assert!(
            rect.includes_portion_of_line(&line1),
            "rectangle should contain line collinear with bottom"
        );
        assert!(
            rect.includes_portion_of_line(&line2),
            "rectangle should contain line collinear with top"
        );
        assert!(
            rect.includes_portion_of_line(&line3),
            "rectangle should contain line collinear with left"
        );
        assert!(
            rect.includes_portion_of_line(&line4),
            "rectangle should contain line collinear with right"
        );
        assert!(
            rect.includes_portion_of_line(&line5),
            "rectangle should contain line intersecting corner"
        );
        assert!(
            rect.includes_portion_of_line(&line6),
            "rectangle should contain line with point touching perimeter"
        );
    }
}
