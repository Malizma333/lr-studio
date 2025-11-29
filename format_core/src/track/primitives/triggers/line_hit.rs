use crate::track::primitives::triggers::Trigger;

#[derive(Debug, Clone, Copy)]
pub struct LineHitTrigger {
    id: u32,
    frame_length: u32,
}

impl LineHitTrigger {
    pub fn new(id: u32, frame_length: u32) -> Self {
        Self { id, frame_length }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn frame_length(&self) -> u32 {
        self.frame_length
    }
}

impl Trigger for LineHitTrigger {}
