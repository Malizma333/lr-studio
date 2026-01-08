use crate::track::{
    primitives::{BackgroundColorEvent, FrameBoundsTrigger},
    trigger::triggered_event::TriggeredEvent,
};

pub type BackgroundColorTrigger = TriggeredEvent<BackgroundColorEvent, FrameBoundsTrigger>;

#[derive(PartialEq, Debug)]
pub struct BackgroundColorGroup {
    triggers: Vec<BackgroundColorTrigger>,
}

impl BackgroundColorGroup {
    pub fn new() -> Self {
        Self {
            triggers: Vec::new(),
        }
    }

    pub fn triggers(&self) -> &Vec<BackgroundColorTrigger> {
        &self.triggers
    }

    pub fn triggers_mut(&mut self) -> &mut Vec<BackgroundColorTrigger> {
        &mut self.triggers
    }
}
