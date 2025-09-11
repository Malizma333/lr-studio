pub struct Vector2Di {
    x: i64,
    y: i64,
}

impl Vector2Di {
    pub fn new(x: i64, y: i64) -> Vector2Di {
        Vector2Di { x, y }
    }

    pub fn copy(v1: &Vector2Di) -> Vector2Di {
        Vector2Di::new(v1.x(), v1.y())
    }

    pub fn x(&self) -> i64 {
        self.x
    }

    pub fn y(&self) -> i64 {
        self.y
    }

    pub fn set_x(&mut self, x: i64) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: i64) {
        self.y = y;
    }

    pub fn add(v1: &Vector2Di, v2: &Vector2Di) -> Vector2Di {
        Vector2Di::new(v1.x() + v2.x(), v1.y() + v2.y())
    }

    pub fn sub(v1: &Vector2Di, v2: &Vector2Di) -> Vector2Di {
        Vector2Di::new(v1.x() - v2.x(), v1.y() - v2.y())
    }

    pub fn equal(v1: &Vector2Di, v2: &Vector2Di) -> bool {
        v1.x() == v2.x() && v1.y() == v2.y()
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
    fn copy() {
        let v1 = Vector2Di::new(5, 10);
        let v2 = Vector2Di::copy(&v1);
        assert_eq!(v2.x(), 5);
        assert_eq!(v2.y(), 10);
    }

    #[test]
    fn addition() {
        let v1 = Vector2Di::new(1, 2);
        let v2 = Vector2Di::new(4, 5);
        let v3 = Vector2Di::add(&v1, &v2);
        assert_eq!(v3.x(), 5);
        assert_eq!(v3.y(), 7);
    }

    #[test]
    fn subtraction() {
        let v1 = Vector2Di::new(1, 5);
        let v2 = Vector2Di::new(4, 2);
        let v3 = Vector2Di::sub(&v1, &v2);
        assert_eq!(v3.x(), -3);
        assert_eq!(v3.y(), 3);
    }

    #[test]
    fn equality() {
        let v1 = Vector2Di::new(1, 5);
        let v2 = Vector2Di::new(4, 3);
        let v3 = Vector2Di::new(1, 5);
        assert_eq!(Vector2Di::equal(&v1, &v2), false);
        assert_eq!(Vector2Di::equal(&v2, &v1), false);
        assert_eq!(Vector2Di::equal(&v1, &v3), true);
        assert_eq!(Vector2Di::equal(&v2, &v3), false);
    }
}
