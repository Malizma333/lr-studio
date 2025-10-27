use std::collections::HashMap;

use crate::entity::{point::state::EntityPointState, registry::EntityPointId};

pub(super) struct EngineState {
    point_states: HashMap<EntityPointId, EntityPointState>,
}

impl Clone for EngineState {
    fn clone(&self) -> Self {
        Self {
            // hashmap clone is implemented as a deep copy
            point_states: self.point_states.clone(),
        }
    }
}

impl EngineState {
    pub fn new() -> Self {
        Self {
            point_states: HashMap::new(),
        }
    }
}
