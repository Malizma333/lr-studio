use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector2Di {
    pub x: i32,
    pub y: i32,
}

impl Add for Vector2Di {
    type Output = Vector2Di;

    fn add(self, rhs: Self) -> Self::Output {
        Vector2Di::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for Vector2Di {
    type Output = Vector2Di;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector2Di::new(self.x - rhs.x, self.y - rhs.y)
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
}

#[cfg(test)]
mod tests {
    use crate::vector2di::Vector2Di;

    #[test]
    fn addition() {
        let v1 = Vector2Di::new(1, 2);
        let v2 = Vector2Di::new(4, 5);
        let v3 = v1 + v2;
        assert!(
            v3.x == 5 && v3.y == 7,
            "vector addition should sum both components"
        );
    }

    #[test]
    fn subtraction() {
        let v1 = Vector2Di::new(1, 5);
        let v2 = Vector2Di::new(4, 2);
        let v3 = v1 - v2;
        assert!(
            v3.x == -3 && v3.y == 3,
            "vector subtraction should difference both components"
        );
    }

    #[test]
    fn equality() {
        let v1 = Vector2Di::new(1, 5);
        let v2 = Vector2Di::new(1, 5);
        let v3 = Vector2Di::new(4, 5);
        let v4 = Vector2Di::new(1, 3);
        assert!(v1 == v2, "vectors with same components should be equal");
        assert!(v2 == v1, "equality should be reflexive");
        assert!(v1 != v3, "vectors with different x's should not be equal");
        assert!(v1 != v4, "vectors with different y's should not be equal");
    }
}
