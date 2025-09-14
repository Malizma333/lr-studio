use std::ops::{Add, Div, Mul, Sub};

use crate::Vector2Di;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector2Df {
    x: f64,
    y: f64,
}

impl Add for Vector2Df {
    type Output = Vector2Df;

    fn add(self, rhs: Self) -> Self::Output {
        Vector2Df::new(self.x() + rhs.x(), self.y() + rhs.y())
    }
}

impl Sub for Vector2Df {
    type Output = Vector2Df;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector2Df::new(self.x() - rhs.x(), self.y() - rhs.y())
    }
}

impl Mul<f64> for Vector2Df {
    type Output = Vector2Df;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector2Df::new(self.x() * rhs, self.y() * rhs)
    }
}

impl Mul<Vector2Df> for f64 {
    type Output = Vector2Df;

    fn mul(self, rhs: Vector2Df) -> Self::Output {
        Vector2Df::new(self * rhs.x(), self * rhs.y())
    }
}

impl Div<f64> for Vector2Df {
    type Output = Vector2Df;

    fn div(self, rhs: f64) -> Self::Output {
        Vector2Df::new(self.x() / rhs, self.y() / rhs)
    }
}

impl Vector2Df {
    pub fn new(x: f64, y: f64) -> Vector2Df {
        Vector2Df { x, y }
    }

    pub fn from(v1: &Vector2Di) -> Vector2Df {
        Vector2Df::new(f64::from(v1.x()), f64::from(v1.y()))
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn set_x(&mut self, x: f64) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: f64) {
        self.y = y;
    }

    /** The zero vector (0.0, 0.0) */
    pub fn zero() -> Vector2Df {
        Vector2Df::new(0.0, 0.0)
    }

    /** The unit vector (1.0, 1.0) */
    pub fn one() -> Vector2Df {
        Vector2Df::new(1.0, 1.0)
    }

    /** Calculates the result of `v1 ⋅ v2` */
    pub fn dot(v1: Vector2Df, v2: Vector2Df) -> f64 {
        v1.x() * v2.x() + v1.y() * v2.y()
    }

    /** Calculates the result of `v1 ⨯ v2` */
    pub fn cross(v1: Vector2Df, v2: Vector2Df) -> f64 {
        v1.x() * v2.y() - v1.y() * v2.x()
    }

    pub fn length_squared(&self) -> f64 {
        self.x() * self.x() + self.y() * self.y()
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn distance(v1: Vector2Df, v2: Vector2Df) -> f64 {
        (v1.clone() - v2.clone()).length()
    }

    /** Creates a new vector rotated 90° clockwise about the origin */
    pub fn rotate_cw(&self) -> Vector2Df {
        Vector2Df::new(self.y(), -self.x())
    }

    /** Creates a new vector rotated 90° counterclockwise about the origin */
    pub fn rotate_ccw(&self) -> Vector2Df {
        Vector2Df::new(-self.y(), self.x())
    }
}

#[cfg(test)]
mod tests {
    use crate::{Vector2Di, vector2df::Vector2Df};

    #[test]
    fn getters() {
        let v1 = Vector2Df::new(1.0, 2.0);
        assert_eq!(v1.x(), 1.0);
        assert_eq!(v1.y(), 2.0);
    }

    #[test]
    fn setters() {
        let mut v1 = Vector2Df::new(1.0, 2.0);
        v1.set_x(3.0);
        v1.set_y(5.0);
        assert_eq!(v1.x(), 3.0);
        assert_eq!(v1.y(), 5.0);
    }

    #[test]
    fn clone() {
        let v1 = Vector2Df::new(5.0, 10.0);
        let mut v2 = v1.clone();
        assert_eq!(v2.x(), 5.0);
        assert_eq!(v2.y(), 10.0);
        v2.set_x(20.0);
        assert_ne!(v1.x(), 20.0);
    }

    #[test]
    fn from() {
        let v1 = Vector2Di::new(1, 3);
        let v2 = Vector2Df::from(&v1);
        assert_eq!(v2.x(), 1.0);
        assert_eq!(v2.y(), 3.0);
    }

    #[test]
    fn zero_vector() {
        let v1 = Vector2Df::zero();
        assert_eq!(v1.x(), 0.0);
        assert_eq!(v1.y(), 0.0);
    }

    #[test]
    fn one_vector() {
        let v1 = Vector2Df::one();
        assert_eq!(v1.x(), 1.0);
        assert_eq!(v1.y(), 1.0);
    }

    #[test]
    fn addition() {
        let v1 = Vector2Df::new(1.0, 2.0);
        let v2 = Vector2Df::new(4.0, 5.0);
        let v3 = v1 + v2;
        assert_eq!(v3.x(), 5.0);
        assert_eq!(v3.y(), 7.0);
    }

    #[test]
    fn subtraction() {
        let v1 = Vector2Df::new(1.0, 5.0);
        let v2 = Vector2Df::new(4.0, 2.0);
        let v3 = v1 - v2;
        assert_eq!(v3.x(), -3.0);
        assert_eq!(v3.y(), 3.0);
    }

    #[test]
    fn scalar_multiplication() {
        let v1 = Vector2Df::new(2.0, 3.0);
        let v2 = v1 * 5.0;
        assert_eq!(v2.x(), 10.0);
        assert_eq!(v2.y(), 15.0);
        let v3 = 3.0 * v1;
        assert_eq!(v3.x(), 6.0);
        assert_eq!(v3.y(), 9.0);
    }

    #[test]
    fn scalar_division() {
        let v1 = Vector2Df::new(2.0, 6.0);
        let v2 = v1 / 2.0;
        assert_eq!(v2.x(), 1.0);
        assert_eq!(v2.y(), 3.0);
    }

    #[test]
    fn equality() {
        let v1 = Vector2Df::new(1.0, 5.0);
        let v2 = Vector2Df::new(4.0, 3.0);
        let v3 = Vector2Df::new(1.0, 5.0);
        assert_eq!(v1 == v2, false);
        assert_eq!(v2 == v1, false);
        assert_eq!(v1 == v3, true);
        assert_eq!(v2 == v3, false);
    }

    #[test]
    fn dot_product() {
        let v1 = Vector2Df::new(1.0, 3.0);
        let v2 = Vector2Df::new(2.0, -4.0);
        let result = Vector2Df::dot(v1, v2);
        assert_eq!(result, -10.0);
    }

    #[test]
    fn cross_product() {
        let v1 = Vector2Df::new(1.0, 3.0);
        let v2 = Vector2Df::new(2.0, -4.0);
        let result = Vector2Df::cross(v1, v2);
        assert_eq!(result, -10.0);
    }

    #[test]
    fn length_squared() {
        let v1 = Vector2Df::new(1.0, -2.0);
        let result = v1.length_squared();
        assert_eq!(result, 5.0);
    }

    #[test]
    fn length() {
        let v1 = Vector2Df::new(3.0, 4.0);
        let result = v1.length();
        assert_eq!(result, 5.0);
    }

    #[test]
    fn distance() {
        let v1 = Vector2Df::new(7.0, -10.0);
        let v2 = Vector2Df::new(2.0, 2.0);
        let result = Vector2Df::distance(v1, v2);
        assert_eq!(result, 13.0);
    }

    #[test]
    fn rotate_clockwise() {
        let v1 = Vector2Df::new(-3.0, 9.0);
        let v2 = v1.rotate_cw();
        assert_eq!(v2.x(), 9.0);
        assert_eq!(v2.y(), 3.0);
    }

    #[test]
    fn rotate_counterclockwise() {
        let v1 = Vector2Df::new(2.0, -7.0);
        let v2 = v1.rotate_ccw();
        assert_eq!(v2.x(), 7.0);
        assert_eq!(v2.y(), 2.0);
    }
}
