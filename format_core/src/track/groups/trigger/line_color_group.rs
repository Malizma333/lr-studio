use crate::track::{
    primitives::{FrameBoundsTrigger, LineColorEvent},
    trigger::triggered_event::{TriggeredEvent, TriggeredEventBuilder},
};

pub type LineColorTrigger = TriggeredEvent<LineColorEvent, FrameBoundsTrigger>;
pub type LineColorTriggerBuilder = TriggeredEventBuilder<LineColorEvent, FrameBoundsTrigger>;

pub struct LineColorGroup {
    triggers: Vec<LineColorTrigger>,
}

impl LineColorGroup {
    pub fn triggers(&self) -> &Vec<LineColorTrigger> {
        &self.triggers
    }
}

pub struct LineColorGroupBuilder {
    triggers: Vec<LineColorTriggerBuilder>,
}

impl LineColorGroupBuilder {
    pub fn new() -> Self {
        LineColorGroupBuilder {
            triggers: Vec::new(),
        }
    }

    pub fn add_trigger(
        &mut self,
        event: LineColorEvent,
        trigger: FrameBoundsTrigger,
    ) -> &mut LineColorTriggerBuilder {
        self.triggers
            .push(LineColorTriggerBuilder::new(event, trigger));
        self.triggers.last_mut().unwrap()
    }

    pub fn get_triggers(&mut self) -> &mut Vec<LineColorTriggerBuilder> {
        &mut self.triggers
    }

    pub fn build(&self) -> Option<LineColorGroup> {
        let mut triggers: Vec<LineColorTrigger> = vec![];

        for trigger_builder in &self.triggers {
            let trigger = trigger_builder.build();
            triggers.push(trigger);
        }

        if triggers.len() == 0 {
            None
        } else {
            Some(LineColorGroup { triggers })
        }
    }
}
