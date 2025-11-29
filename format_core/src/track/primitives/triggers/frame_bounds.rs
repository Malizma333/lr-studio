use crate::track::primitives::triggers::Trigger;

#[derive(Debug, Clone, Copy)]
pub struct FrameBoundsTrigger {
    start: u32,
    end: u32,
}

impl FrameBoundsTrigger {
    pub fn new(start: u32, end: u32) -> Self {
        Self { start, end }
    }

    pub fn start(&self) -> u32 {
        self.start
    }

    pub fn end(&self) -> u32 {
        self.end
    }
}

impl Trigger for FrameBoundsTrigger {}
