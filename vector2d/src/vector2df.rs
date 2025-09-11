pub struct Vector2Df {
    x: f64,
    y: f64,
}

impl Vector2Df {
    pub fn new(x: f64, y: f64) -> Vector2Df {
        Vector2Df { x, y }
    }

    pub fn copy(v1: &Vector2Df) -> Vector2Df {
        Vector2Df::new(v1.x(), v1.y())
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

    pub fn add(v1: &Vector2Df, v2: &Vector2Df) -> Vector2Df {
        Vector2Df::new(v1.x() + v2.x(), v1.y() + v2.y())
    }

    pub fn sub(v1: &Vector2Df, v2: &Vector2Df) -> Vector2Df {
        Vector2Df::new(v1.x() - v2.x(), v1.y() - v2.y())
    }

    pub fn equal(v1: &Vector2Df, v2: &Vector2Df) -> bool {
        v1.x() == v2.x() && v1.y() == v2.y()
    }

    pub fn scale(v1: &Vector2Df, s: f64) -> Vector2Df {
        Vector2Df::new(v1.x() * s, v1.y() * s)
    }

    pub fn dot(v1: &Vector2Df, v2: &Vector2Df) -> f64 {
        v1.x() * v2.x() + v1.y() * v2.y()
    }

    pub fn cross(v1: &Vector2Df, v2: &Vector2Df) -> f64 {
        v1.x() * v2.y() - v1.y() * v2.x()
    }

    pub fn length_squared(v1: &Vector2Df) -> f64 {
        v1.x() * v1.x() + v1.y() * v1.y()
    }

    pub fn length(v1: &Vector2Df) -> f64 {
        (v1.x() * v1.x() + v1.y() * v1.y()).sqrt()
    }

    pub fn distance(v1: &Vector2Df, v2: &Vector2Df) -> f64 {
        ((v1.x() - v2.x()) * (v1.x() - v2.x()) + (v1.y() - v2.y()) * (v1.y() - v2.y())).sqrt()
    }

    pub fn rotate_cw(v1: &Vector2Df) -> Vector2Df {
        Vector2Df::new(v1.y(), -v1.x())
    }

    pub fn rotate_ccw(v1: &Vector2Df) -> Vector2Df {
        Vector2Df::new(-v1.y(), v1.x())
    }
}

#[cfg(test)]
mod tests {
    use crate::vector2df::Vector2Df;

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
    fn copy() {
        let v1 = Vector2Df::new(5.0, 10.0);
        let v2 = Vector2Df::copy(&v1);
        assert_eq!(v2.x(), 5.0);
        assert_eq!(v2.y(), 10.0);
    }

    #[test]
    fn addition() {
        let v1 = Vector2Df::new(1.0, 2.0);
        let v2 = Vector2Df::new(4.0, 5.0);
        let v3 = Vector2Df::add(&v1, &v2);
        assert_eq!(v3.x(), 5.0);
        assert_eq!(v3.y(), 7.0);
    }

    #[test]
    fn subtraction() {
        let v1 = Vector2Df::new(1.0, 5.0);
        let v2 = Vector2Df::new(4.0, 2.0);
        let v3 = Vector2Df::sub(&v1, &v2);
        assert_eq!(v3.x(), -3.0);
        assert_eq!(v3.y(), 3.0);
    }

    #[test]
    fn scalar_multiplication() {
        let v1 = Vector2Df::new(2.0, 3.0);
        let v2 = Vector2Df::scale(&v1, 5.0);
        assert_eq!(v2.x(), 10.0);
        assert_eq!(v2.y(), 15.0);
    }

    #[test]
    fn equality() {
        let v1 = Vector2Df::new(1.0, 5.0);
        let v2 = Vector2Df::new(4.0, 3.0);
        let v3 = Vector2Df::new(1.0, 5.0);
        assert_eq!(Vector2Df::equal(&v1, &v2), false);
        assert_eq!(Vector2Df::equal(&v2, &v1), false);
        assert_eq!(Vector2Df::equal(&v1, &v3), true);
        assert_eq!(Vector2Df::equal(&v2, &v3), false);
    }

    #[test]
    fn dot_product() {
        let v1 = Vector2Df::new(1.0, 3.0);
        let v2 = Vector2Df::new(2.0, -4.0);
        let result = Vector2Df::dot(&v1, &v2);
        assert_eq!(result, -10.0);
    }

    #[test]
    fn cross_product() {
        let v1 = Vector2Df::new(1.0, 3.0);
        let v2 = Vector2Df::new(2.0, -4.0);
        let result = Vector2Df::cross(&v1, &v2);
        assert_eq!(result, -10.0);
    }

    #[test]
    fn length_squared() {
        let v1 = Vector2Df::new(1.0, -2.0);
        let result = Vector2Df::length_squared(&v1);
        assert_eq!(result, 5.0);
    }

    #[test]
    fn length() {
        let v1 = Vector2Df::new(3.0, 4.0);
        let result = Vector2Df::length(&v1);
        assert_eq!(result, 5.0);
    }

    #[test]
    fn distance() {
        let v1 = Vector2Df::new(7.0, -10.0);
        let v2 = Vector2Df::new(2.0, 2.0);
        let result = Vector2Df::distance(&v1, &v2);
        assert_eq!(result, 13.0);
    }

    #[test]
    fn rotate_clockwise() {
        let v1 = Vector2Df::new(-3.0, 9.0);
        let v2 = Vector2Df::rotate_cw(&v1);
        assert_eq!(v2.x(), 9.0);
        assert_eq!(v2.y(), 3.0);
    }

    #[test]
    fn rotate_counterclockwise() {
        let v1 = Vector2Df::new(2.0, -7.0);
        let v2 = Vector2Df::rotate_ccw(&v1);
        assert_eq!(v2.x(), 7.0);
        assert_eq!(v2.y(), 2.0);
    }
}
