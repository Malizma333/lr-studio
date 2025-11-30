use crate::track::{
    primitives::{CameraZoomEvent, LineHitTrigger},
    trigger::triggered_event::{TriggeredEvent, TriggeredEventBuilder},
};

pub type LegacyCameraZoomTrigger = TriggeredEvent<CameraZoomEvent, LineHitTrigger>;
pub type LegacyCameraZoomTriggerBuilder = TriggeredEventBuilder<CameraZoomEvent, LineHitTrigger>;

pub struct LegacyCameraZoomGroup {
    triggers: Vec<LegacyCameraZoomTrigger>,
}

impl LegacyCameraZoomGroup {
    pub fn triggers(&self) -> &Vec<LegacyCameraZoomTrigger> {
        &self.triggers
    }
}

pub struct LegacyCameraZoomGroupBuilder {
    triggers: Vec<LegacyCameraZoomTriggerBuilder>,
}

impl LegacyCameraZoomGroupBuilder {
    pub fn new() -> Self {
        LegacyCameraZoomGroupBuilder {
            triggers: Vec::new(),
        }
    }

    pub fn add_trigger(
        &mut self,
        event: CameraZoomEvent,
        trigger: LineHitTrigger,
    ) -> &mut LegacyCameraZoomTriggerBuilder {
        self.triggers
            .push(LegacyCameraZoomTriggerBuilder::new(event, trigger));
        self.triggers.last_mut().unwrap()
    }

    pub fn get_triggers(&mut self) -> &mut Vec<LegacyCameraZoomTriggerBuilder> {
        &mut self.triggers
    }

    pub fn build(&self) -> Option<LegacyCameraZoomGroup> {
        let mut triggers: Vec<LegacyCameraZoomTrigger> = vec![];

        for trigger_builder in &self.triggers {
            let trigger = trigger_builder.build();
            triggers.push(trigger);
        }

        if triggers.len() == 0 {
            None
        } else {
            Some(LegacyCameraZoomGroup { triggers })
        }
    }
}
