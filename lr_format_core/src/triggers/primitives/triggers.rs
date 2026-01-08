mod frame_bounds;
mod line_hit;

pub use frame_bounds::FrameBoundsTrigger;
pub use line_hit::LineHitTrigger;

pub trait Trigger: Clone {}
