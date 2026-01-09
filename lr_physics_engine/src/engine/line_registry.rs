use geometry::Point;
use lr_physics_grid::{Grid, GridLineId, GridVersion};
use std::collections::HashMap;

use crate::PhysicsLine;

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
pub struct LineId(GridLineId);

pub(crate) struct LineRegistry {
    grid: Grid,
    line_lookup: HashMap<LineId, PhysicsLine>,
}

impl LineRegistry {
    pub(crate) fn new(grid_version: GridVersion) -> Self {
        LineRegistry {
            grid: Grid::new(grid_version),
            line_lookup: HashMap::new(),
        }
    }

    pub(crate) fn set_grid_version(&mut self, new_version: GridVersion) {
        self.grid.set_version(new_version);
    }

    pub(crate) fn add_line(&mut self, line: PhysicsLine) -> LineId {
        let id = LineId(self.grid.add_line(line.endpoints()));
        self.line_lookup.insert(id, line);
        id
    }

    pub(crate) fn get_line(&self, id: LineId) -> Option<&PhysicsLine> {
        self.line_lookup.get(&id)
    }

    /// Replaces a line with a new line, preserving its id
    pub(crate) fn replace_line(&mut self, id: LineId, new_line: PhysicsLine) {
        self.grid.update_line(id.0, new_line.endpoints());
        self.line_lookup.insert(id, new_line);
    }

    pub(crate) fn remove_line(&mut self, id: LineId) {
        let line = self.line_lookup.remove(&id);
        if line.is_some() {
            self.grid.remove_line(id.0);
        }
    }

    /// Uses the grid to collect all lines around a point
    pub(crate) fn lines_near_point(&self, point: Point) -> Vec<&PhysicsLine> {
        let line_ids = self.grid.get_lines_near_point(point);
        line_ids
            .iter()
            .map(|id| &self.line_lookup[&LineId(*id)])
            .collect()
    }
}
