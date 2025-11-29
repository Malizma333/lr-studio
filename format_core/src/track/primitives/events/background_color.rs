use crate::track::{RGBColor, primitives::events::Event};

#[derive(Debug, Clone, Copy)]
pub struct BackgroundColorEvent {
    color: RGBColor,
}

impl BackgroundColorEvent {
    pub fn new(color: RGBColor) -> Self {
        Self { color }
    }

    pub fn color(&self) -> RGBColor {
        self.color
    }
}

impl Event for BackgroundColorEvent {}
