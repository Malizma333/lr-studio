use std::collections::HashMap;

use geometry::{Line, Point};
use lr_physics_grid::{Grid, GridLineId, GridVersion};

use crate::PhysicsLine;

pub struct LineStore {
    grid: Grid,
    line_lookup: HashMap<GridLineId, PhysicsLine>,
}

impl LineStore {
    pub fn new(grid_version: GridVersion) -> Self {
        LineStore {
            grid: Grid::new(grid_version),
            line_lookup: HashMap::new(),
        }
    }

    pub fn update_grid_version(&mut self, new_version: GridVersion) {
        self.grid.update_version(new_version);
    }

    // TODO provide nice api for these to easily lookup which frame needs to be invalidated (perhaps providing which grid cells were changed?)

    pub fn add_line(&mut self, line: PhysicsLine) -> GridLineId {
        let id = self.grid.add_line(line.endpoints());
        self.line_lookup.insert(id, line);
        id
    }

    pub fn update_line(&mut self, line_id: GridLineId, new_points: Line) {
        let line = self.line_lookup.get_mut(&line_id);
        if let Some(line) = line {
            self.grid.update_line(line_id, new_points);
            line.set_endpoints(new_points);
        }
    }

    pub fn remove_line(&mut self, line_id: GridLineId) {
        let line = self.line_lookup.remove(&line_id);
        if line.is_some() {
            self.grid.remove_line(line_id);
        }
    }

    pub fn lines_near_point(&self, point: Point) -> Vec<&PhysicsLine> {
        let line_ids = self.grid.get_lines_near_point(point);
        line_ids.iter().map(|id| &self.line_lookup[&id]).collect()
    }
}
