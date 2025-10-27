use crate::entity::point::{snapshot::EntityPointSnapshot, state::EntityPointState};

pub(crate) mod snapshot;
pub(crate) mod state;
pub(crate) mod template;

pub(crate) struct EntityPoint {
    contact: bool,
    contact_friction: f64,
    air_friction: f64,
}

impl EntityPoint {
    pub fn get_snapshot(&self) -> EntityPointSnapshot {
        EntityPointSnapshot::new(
            position,
            velocity,
            previous_position,
            self.contact_friction,
            self.air_friction,
            self.contact,
        )
    }
}
