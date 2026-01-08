use crate::track::{
    primitives::{CameraZoomEvent, LineHitTrigger},
    trigger::triggered_event::TriggeredEvent,
};

pub type LegacyCameraZoomTrigger = TriggeredEvent<CameraZoomEvent, LineHitTrigger>;

#[derive(PartialEq, Debug)]
pub struct LegacyCameraZoomGroup {
    triggers: Vec<LegacyCameraZoomTrigger>,
}

impl LegacyCameraZoomGroup {
    pub fn new() -> Self {
        Self {
            triggers: Vec::new(),
        }
    }

    pub fn triggers(&self) -> &Vec<LegacyCameraZoomTrigger> {
        &self.triggers
    }

    pub fn triggers_mut(&mut self) -> &mut Vec<LegacyCameraZoomTrigger> {
        &mut self.triggers
    }
}
