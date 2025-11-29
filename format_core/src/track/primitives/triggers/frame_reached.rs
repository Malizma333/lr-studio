use crate::track::primitives::triggers::Trigger;

#[derive(Debug, Clone, Copy)]
pub struct FrameReachedTrigger {
    frame: u32,
}

impl FrameReachedTrigger {
    pub fn new(frame: u32) -> Self {
        Self { frame }
    }

    pub fn frame(&self) -> u32 {
        self.frame
    }
}

impl Trigger for FrameReachedTrigger {}
