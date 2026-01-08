use crate::track::{
    primitives::{FrameBoundsTrigger, LineColorEvent},
    trigger::triggered_event::TriggeredEvent,
};

pub type LineColorTrigger = TriggeredEvent<LineColorEvent, FrameBoundsTrigger>;

#[derive(PartialEq, Debug)]
pub struct LineColorGroup {
    triggers: Vec<LineColorTrigger>,
}

impl LineColorGroup {
    pub fn new() -> Self {
        Self {
            triggers: Vec::new(),
        }
    }

    pub fn triggers(&self) -> &Vec<LineColorTrigger> {
        &self.triggers
    }

    pub fn triggers_mut(&mut self) -> &mut Vec<LineColorTrigger> {
        &mut self.triggers
    }
}
