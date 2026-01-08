use crate::track::{
    primitives::{CameraZoomEvent, FrameBoundsTrigger},
    trigger::triggered_event::TriggeredEvent,
};

pub type CameraZoomTrigger = TriggeredEvent<CameraZoomEvent, FrameBoundsTrigger>;

#[derive(PartialEq, Debug)]
pub struct CameraZoomGroup {
    triggers: Vec<CameraZoomTrigger>,
}

impl CameraZoomGroup {
    pub fn new() -> Self {
        Self {
            triggers: Vec::new(),
        }
    }

    pub fn triggers(&self) -> &Vec<CameraZoomTrigger> {
        &self.triggers
    }

    pub fn triggers_mut(&mut self) -> &mut Vec<CameraZoomTrigger> {
        &mut self.triggers
    }
}
