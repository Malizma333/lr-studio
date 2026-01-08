use crate::track::primitives::{Event, Trigger};

#[derive(PartialEq, Debug)]
pub struct TriggeredEvent<E: Event, T: Trigger> {
    event: E,
    trigger: T,
}

impl<E: Event, T: Trigger> TriggeredEvent<E, T> {
    pub fn new(event: E, trigger: T) -> Self {
        Self { event, trigger }
    }

    pub fn event(&self) -> &E {
        &self.event
    }

    pub fn trigger(&self) -> &T {
        &self.trigger
    }
}
