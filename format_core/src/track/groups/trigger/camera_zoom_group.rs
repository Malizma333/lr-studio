use crate::track::{
    primitives::{CameraZoomEvent, FrameBoundsTrigger},
    trigger::triggered_event::{TriggeredEvent, TriggeredEventBuilder},
};

pub type CameraZoomTrigger = TriggeredEvent<CameraZoomEvent, FrameBoundsTrigger>;
pub type CameraZoomTriggerBuilder = TriggeredEventBuilder<CameraZoomEvent, FrameBoundsTrigger>;

pub struct CameraZoomGroup {
    triggers: Vec<CameraZoomTrigger>,
}

impl CameraZoomGroup {
    pub fn triggers(&self) -> &Vec<CameraZoomTrigger> {
        &self.triggers
    }
}

pub struct CameraZoomGroupBuilder {
    triggers: Vec<CameraZoomTriggerBuilder>,
}

impl CameraZoomGroupBuilder {
    pub fn new() -> Self {
        CameraZoomGroupBuilder {
            triggers: Vec::new(),
        }
    }

    pub fn add_trigger(
        &mut self,
        event: CameraZoomEvent,
        trigger: FrameBoundsTrigger,
    ) -> &mut CameraZoomTriggerBuilder {
        self.triggers
            .push(CameraZoomTriggerBuilder::new(event, trigger));
        self.triggers.last_mut().unwrap()
    }

    pub fn get_triggers(&mut self) -> &mut Vec<CameraZoomTriggerBuilder> {
        &mut self.triggers
    }

    pub fn build(&self) -> Option<CameraZoomGroup> {
        let mut triggers: Vec<CameraZoomTrigger> = vec![];

        for trigger_builder in &self.triggers {
            let trigger = trigger_builder.build();
            triggers.push(trigger);
        }

        if triggers.len() == 0 {
            None
        } else {
            Some(CameraZoomGroup { triggers })
        }
    }
}
