use vector2d::Vector2Df;

pub struct InitialProperties {
    start_offset: Vector2Df,
    start_velocity: Vector2Df,
}

impl InitialProperties {
    pub fn new() -> Self {
        InitialProperties {
            start_offset: Vector2Df::zero(),
            start_velocity: Vector2Df::zero(),
        }
    }

    pub fn set_start_offset(&mut self, start_offset: Vector2Df) {
        self.start_offset = start_offset;
    }

    pub fn set_start_velocity(&mut self, start_velocity: Vector2Df) {
        self.start_velocity = start_velocity;
    }

    pub(crate) fn start_offset(&self) -> Vector2Df {
        self.start_offset
    }

    pub(crate) fn start_velocity(&self) -> Vector2Df {
        self.start_velocity
    }
}
