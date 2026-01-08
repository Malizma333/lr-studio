use std::ops::{Div, Mul};

use vector2d::{Vector2Df, Vector2Di};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    x: f64,
    y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }

    pub fn zero() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn set_x(&mut self, x: f64) {
        self.x = x;
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn set_y(&mut self, y: f64) {
        self.y = y;
    }

    pub fn vector_from(&self, other: Point) -> Vector2Df {
        Vector2Df::new(self.x - other.x, self.y - other.y)
    }

    pub fn translated_by(&self, amount: Vector2Df) -> Point {
        Point::new(self.x + amount.x(), self.y + amount.y())
    }

    pub fn distance_squared_from(&self, other: Point) -> f64 {
        self.vector_from(other).length_squared()
    }

    pub fn distance_from(&self, other: Point) -> f64 {
        self.vector_from(other).length()
    }
}

impl Mul<Point> for f64 {
    type Output = Point;
    fn mul(self, rhs: Point) -> Self::Output {
        Point::new(self * rhs.x, self * rhs.y)
    }
}

impl Mul<f64> for Point {
    type Output = Point;

    fn mul(self, rhs: f64) -> Self::Output {
        Point::new(self.x * rhs, self.y * rhs)
    }
}

impl Div<f64> for Point {
    type Output = Point;

    fn div(self, rhs: f64) -> Self::Output {
        Point::new(self.x / rhs, self.y / rhs)
    }
}

impl From<Point> for Vector2Df {
    fn from(value: Point) -> Self {
        Vector2Df::new(value.x, value.y)
    }
}

impl From<Vector2Df> for Point {
    fn from(value: Vector2Df) -> Self {
        Point::new(value.x(), value.y())
    }
}

impl From<Vector2Di> for Point {
    fn from(value: Vector2Di) -> Self {
        Point::new(f64::from(value.x()), f64::from(value.y()))
    }
}
