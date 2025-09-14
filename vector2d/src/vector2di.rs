use std::ops::{Add, Sub};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector2Di {
    x: i32,
    y: i32,
}

impl Add for Vector2Di {
    type Output = Vector2Di;

    fn add(self, rhs: Self) -> Self::Output {
        Vector2Di::new(self.x() + rhs.x(), self.y() + rhs.y())
    }
}

impl Sub for Vector2Di {
    type Output = Vector2Di;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector2Di::new(self.x() - rhs.x(), self.y() - rhs.y())
    }
}

impl Vector2Di {
    pub fn new(x: i32, y: i32) -> Vector2Di {
        Vector2Di { x, y }
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }

    pub fn set_x(&mut self, x: i32) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: i32) {
        self.y = y;
    }
}

#[cfg(test)]
mod tests {
    use crate::vector2di::Vector2Di;

    #[test]
    fn getters() {
        let v1 = Vector2Di::new(1, 2);
        assert_eq!(v1.x(), 1);
        assert_eq!(v1.y(), 2);
    }

    #[test]
    fn setters() {
        let mut v1 = Vector2Di::new(1, 2);
        v1.set_x(3);
        v1.set_y(5);
        assert_eq!(v1.x(), 3);
        assert_eq!(v1.y(), 5);
    }

    #[test]
    fn clone() {
        let v1 = Vector2Di::new(5, 10);
        let mut v2 = v1.clone();
        assert_eq!(v2.x(), 5);
        assert_eq!(v2.y(), 10);
        v2.set_x(20);
        assert_ne!(v1.x(), 20);
    }

    #[test]
    fn addition() {
        let v1 = Vector2Di::new(1, 2);
        let v2 = Vector2Di::new(4, 5);
        let v3 = v1 + v2;
        assert_eq!(v3.x(), 5);
        assert_eq!(v3.y(), 7);
    }

    #[test]
    fn subtraction() {
        let v1 = Vector2Di::new(1, 5);
        let v2 = Vector2Di::new(4, 2);
        let v3 = v1 - v2;
        assert_eq!(v3.x(), -3);
        assert_eq!(v3.y(), 3);
    }

    #[test]
    fn equality() {
        let v1 = Vector2Di::new(1, 5);
        let v2 = Vector2Di::new(4, 3);
        let v3 = Vector2Di::new(1, 5);
        assert_eq!(v1 == v2, false);
        assert_eq!(v2 == v1, false);
        assert_eq!(v1 == v3, true);
        assert_eq!(v2 == v3, false);
    }
}
