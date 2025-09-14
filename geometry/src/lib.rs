mod line;
mod point;
mod rectangle;

pub use line::Line;
pub use point::Point;
pub use rectangle::Rectangle;

pub fn between(bound0: f64, value: f64, bound1: f64) -> bool {
    bound0.min(bound1) <= value && value <= bound0.max(bound1)
}
