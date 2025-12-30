use std::collections::HashMap;

use spatial_grid::{Grid, GridVersion};

use crate::{
    Engine,
    engine::{
        defaults::{default_get_gravity_at_time, default_get_skeleton_frozen_at_time},
        state::EngineState,
    },
    entity::registry::EntityRegistry,
};

pub struct EngineBuilder {
    grid_version: GridVersion,
}

impl EngineBuilder {
    pub fn new(grid_version: GridVersion) -> EngineBuilder {
        EngineBuilder { grid_version }
    }

    pub fn build(&self) -> Engine {
        let initial_state = EngineState::new();

        Engine {
            grid: Grid::new(self.grid_version.clone()),
            line_lookup: HashMap::new(),
            registry: EntityRegistry::new(),
            state_snapshots: vec![initial_state.clone()],
            initial_state,
            get_gravity_at_time: default_get_gravity_at_time,
            get_skeleton_frozen_at_time: default_get_skeleton_frozen_at_time,
        }
    }
}
