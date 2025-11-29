use crate::track::primitives::events::Event;

#[derive(Debug, Clone, Copy)]
pub struct CameraZoomEvent {
    zoom: f64,
}

impl CameraZoomEvent {
    pub fn new(zoom: f64) -> Self {
        Self { zoom }
    }

    pub fn zoom(&self) -> f64 {
        self.zoom
    }
}

impl Event for CameraZoomEvent {}
