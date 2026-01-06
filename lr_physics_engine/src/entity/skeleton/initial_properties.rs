use vector2d::Vector2Df;

pub struct InitialProperties {
    start_position: Vector2Df,
    start_velocity: Vector2Df,
}

impl InitialProperties {
    pub fn new() -> Self {
        InitialProperties {
            start_position: Vector2Df::zero(),
            start_velocity: Vector2Df::zero(),
        }
    }

    pub fn set_start_position(&mut self, start_position: Vector2Df) {
        self.start_position = start_position;
    }

    pub fn set_start_velocity(&mut self, start_velocity: Vector2Df) {
        self.start_velocity = start_velocity;
    }

    pub(crate) fn start_position(&self) -> Vector2Df {
        self.start_position
    }

    pub(crate) fn start_velocity(&self) -> Vector2Df {
        self.start_velocity
    }
}
