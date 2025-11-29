use crate::track::{RGBColor, primitives::events::Event};

#[derive(Debug, Clone, Copy)]
pub struct LineColorEvent {
    color: RGBColor,
}

impl LineColorEvent {
    pub fn new(color: RGBColor) -> Self {
        Self { color }
    }

    pub fn color(&self) -> RGBColor {
        self.color
    }
}

impl Event for LineColorEvent {}
