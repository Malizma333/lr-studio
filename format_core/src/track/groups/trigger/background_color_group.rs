use crate::track::{
    primitives::{BackgroundColorEvent, FrameBoundsTrigger},
    trigger::triggered_event::{TriggeredEvent, TriggeredEventBuilder},
};

pub type BackgroundColorTrigger = TriggeredEvent<BackgroundColorEvent, FrameBoundsTrigger>;
pub type BackgroundColorTriggerBuilder =
    TriggeredEventBuilder<BackgroundColorEvent, FrameBoundsTrigger>;

pub struct BackgroundColorGroup {
    triggers: Vec<BackgroundColorTrigger>,
}

impl BackgroundColorGroup {
    pub fn triggers(&self) -> &Vec<BackgroundColorTrigger> {
        &self.triggers
    }
}

pub struct BackgroundColorGroupBuilder {
    triggers: Vec<BackgroundColorTriggerBuilder>,
}

impl BackgroundColorGroupBuilder {
    pub fn new() -> Self {
        BackgroundColorGroupBuilder {
            triggers: Vec::new(),
        }
    }

    pub fn add_trigger(
        &mut self,
        event: BackgroundColorEvent,
        trigger: FrameBoundsTrigger,
    ) -> &mut BackgroundColorTriggerBuilder {
        self.triggers
            .push(BackgroundColorTriggerBuilder::new(event, trigger));
        self.triggers.last_mut().unwrap()
    }

    pub fn get_triggers(&mut self) -> &mut Vec<BackgroundColorTriggerBuilder> {
        &mut self.triggers
    }

    pub fn build(&self) -> Option<BackgroundColorGroup> {
        let mut triggers: Vec<BackgroundColorTrigger> = vec![];

        for trigger_builder in &self.triggers {
            let trigger = trigger_builder.build();
            triggers.push(trigger);
        }

        if triggers.len() == 0 {
            None
        } else {
            Some(BackgroundColorGroup { triggers })
        }
    }
}
