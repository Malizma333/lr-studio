use crate::track::primitives::{Event, Trigger};

pub struct TriggeredEvent<E: Event, T: Trigger> {
    event: E,
    trigger: T,
}

impl<E: Event, T: Trigger> TriggeredEvent<E, T> {
    pub fn event(&self) -> &E {
        &self.event
    }

    pub fn trigger(&self) -> &T {
        &self.trigger
    }
}

pub struct TriggeredEventBuilder<E: Event, T: Trigger> {
    event: E,
    trigger: T,
}

impl<E: Event, T: Trigger> TriggeredEventBuilder<E, T> {
    pub fn new(event: E, trigger: T) -> TriggeredEventBuilder<E, T> {
        TriggeredEventBuilder { event, trigger }
    }

    pub fn build(&self) -> TriggeredEvent<E, T> {
        TriggeredEvent {
            event: self.event.clone(),
            trigger: self.trigger.clone(),
        }
    }
}
