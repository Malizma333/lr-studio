use crate::{
    entity::registry::{EntityRegistry, EntitySkeletonId},
    grid::{Grid, LineId},
    line::Hitbox,
};
use geometry::Line;
use std::collections::HashMap;
use vector2d::Vector2Df;
mod builder;
mod defaults;
mod moment;
mod state;
mod version;
pub use builder::EngineBuilder;
pub use moment::PhysicsMoment;
pub use state::EngineState;
pub use version::EngineVersion;

const GRAVITY_MULTIPLIER: f64 = 0.175;

pub struct Engine {
    grid: Grid,
    line_lookup: HashMap<LineId, Box<dyn Hitbox>>,
    registry: EntityRegistry,
    initial_state: EngineState,
    state_snapshots: Vec<EngineState>,
    get_gravity_at_time: fn(u32) -> Vector2Df,
    get_skeleton_frozen_at_time: fn(EntitySkeletonId, u32) -> bool,
}

impl Engine {
    pub fn view_frame(&mut self, frame: u32) -> &EngineState {
        self.fill_snapshots_up_to_frame(frame);
        self.state_snapshots
            .get(frame as usize)
            .unwrap_or(&self.initial_state)
    }

    pub fn view_moment(&mut self, frame: u32, moment: PhysicsMoment) -> EngineState {
        self.fill_snapshots_up_to_frame(frame);
        let target_frame_state = self
            .state_snapshots
            .get(frame as usize)
            .unwrap_or(&self.initial_state);
        self.get_next_state(target_frame_state.clone(), Some(moment))
    }

    pub fn define_gravity(&mut self, function: fn(u32) -> Vector2Df) {
        self.get_gravity_at_time = function;
    }

    pub fn define_skeleton_frozen(&mut self, function: fn(EntitySkeletonId, u32) -> bool) {
        self.get_skeleton_frozen_at_time = function;
    }

    // TODO transaction api for batch line updates
    pub fn create_line(&mut self, line: Box<dyn Hitbox>) -> LineId {
        let line_points = &Line::from_tuple(line.properties().endpoints());
        let id = self.grid.add_line(line_points);
        self.line_lookup.insert(id, line);
        self.invalidate_frames();
        id
    }

    pub fn move_line(&mut self, line_id: LineId, new_points: Line) {
        let line = self.line_lookup.get(&line_id);
        if let Some(line) = line {
            let line_points = &Line::from_tuple(line.properties().endpoints());
            self.grid.move_line(line_id, line_points, &new_points);
            self.invalidate_frames();
        }
    }

    pub fn delete_line(&mut self, line_id: LineId) {
        let line = self.line_lookup.remove(&line_id);
        if let Some(line) = line {
            let line_points = &Line::from_tuple(line.properties().endpoints());
            self.grid.remove_line(line_id, line_points);
            self.invalidate_frames();
        }
    }

    // TODO this could be more sophisticated, but for now just reset to initial frame
    fn invalidate_frames(&mut self) {
        self.state_snapshots.truncate(1);
    }

    pub fn registry(&mut self) -> &mut EntityRegistry {
        &mut self.registry
    }

    fn fill_snapshots_up_to_frame(&mut self, target_frame: u32) {
        let mut current_state = self
            .state_snapshots
            .last()
            .unwrap_or(&self.initial_state)
            .clone();

        while (self.state_snapshots.len() as u32) < target_frame + 1 {
            let next_state = self.get_next_state(current_state, None);
            self.state_snapshots.push(next_state.clone());
            current_state = next_state.clone();
        }
    }

    // TODO multithreading multirider support
    fn get_next_state(
        &self,
        current_state: EngineState,
        moment: Option<PhysicsMoment>,
    ) -> EngineState {
        todo!("transform the current state of entities to the next state by simulating a frame")
    }
}
