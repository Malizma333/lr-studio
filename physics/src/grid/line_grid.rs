use crate::grid::grid_cell::{CELL_SIZE, CellKey, GridCell};
use geometry::{Line, Point};
use std::collections::{BTreeSet, HashMap};
use vector2d::Vector2Df;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct LineId(u32);

#[derive(Clone)]
pub enum GridVersion {
    V6_0,
    V6_1,
    V6_2,
}

pub struct Grid {
    version: GridVersion,
    cells: HashMap<CellKey, BTreeSet<LineId>>,
    ids: BTreeSet<LineId>,
}

impl Grid {
    pub fn new(version: GridVersion) -> Grid {
        Grid {
            version,
            cells: HashMap::new(),
            ids: BTreeSet::new(),
        }
    }

    fn get_next_id(&mut self) -> LineId {
        let last_id = self.ids.last().unwrap_or(&LineId(0));
        let next_id = last_id.0 + 1;
        self.ids.insert(LineId(next_id));
        LineId(next_id)
    }

    fn free_id(&mut self, id: LineId) {
        self.ids.remove(&id);
    }

    fn register(&mut self, line_id: LineId, position: &GridCell) {
        let cell_key = position.get_key();
        self.cells
            .entry(cell_key)
            .or_insert_with(BTreeSet::new)
            .insert(line_id);
    }

    fn unregister(&mut self, line_id: LineId, position: &GridCell) {
        let cell_key = position.get_key();
        if let Some(cell) = self.cells.get_mut(&cell_key) {
            cell.remove(&line_id);
        }
    }

    pub fn add_line(&mut self, endpoints: &Line) -> LineId {
        let id = self.get_next_id();
        let cell_positions = self.get_cell_positions_along(&endpoints);
        for position in cell_positions {
            self.register(id, &position);
        }
        id
    }

    pub fn remove_line(&mut self, id: LineId, endpoints: &Line) {
        self.free_id(id);
        let cell_positions = self.get_cell_positions_along(&endpoints);
        for position in cell_positions {
            self.unregister(id, &position);
        }
    }

    pub fn move_line(&mut self, id: LineId, old_endpoints: &Line, new_endpoints: &Line) {
        let cell_positions = self.get_cell_positions_along(&old_endpoints);
        for position in cell_positions {
            self.unregister(id, &position);
        }

        let new_cell_positions = self.get_cell_positions_along(&new_endpoints);
        for position in new_cell_positions {
            self.register(id, &position);
        }
    }

    fn get_next_position(&self, current_position: Point, endpoints: &Line) -> Point {
        let current_cell = GridCell::new(current_position);
        let endpoint_vector = endpoints.get_vector();

        let mut delta_x = if endpoint_vector.x > 0.0 {
            f64::from(CELL_SIZE) - current_cell.remainder().x
        } else {
            -1.0 - current_cell.remainder().x
        };

        let mut delta_y = if endpoint_vector.y > 0.0 {
            f64::from(CELL_SIZE) - current_cell.remainder().y
        } else {
            -1.0 - current_cell.remainder().y
        };

        if matches!(self.version, GridVersion::V6_2) {
            if current_cell.position().x < 0 {
                delta_x = if endpoint_vector.x > 0.0 {
                    f64::from(CELL_SIZE) + current_cell.remainder().x
                } else {
                    -(f64::from(CELL_SIZE) + current_cell.remainder().x)
                }
            }
            if current_cell.position().y < 0 {
                delta_y = if endpoint_vector.y > 0.0 {
                    f64::from(CELL_SIZE) + current_cell.remainder().y
                } else {
                    -(f64::from(CELL_SIZE) + current_cell.remainder().y)
                }
            }
        }

        if endpoint_vector.x == 0.0 {
            Point::new(current_position.x, current_position.y + delta_y)
        } else if endpoint_vector.y == 0.0 {
            Point::new(current_position.x + delta_x, current_position.y)
        } else if matches!(self.version, GridVersion::V6_1) {
            let slope = endpoint_vector.y / endpoint_vector.x;
            let y_intercept = endpoints.p0().y - slope * endpoints.p0().x;
            let next_x = ((current_position.y + delta_y - y_intercept) / slope).round();
            let next_y = (slope * (current_position.x + delta_x) + y_intercept).round();
            if (next_y - current_position.y).abs() < delta_y.abs() {
                Point::new(current_position.x + delta_x, next_y)
            } else if (next_y - current_position.y).abs() == delta_y.abs() {
                Point::new(current_position.x + delta_x, current_position.y + delta_y)
            } else {
                Point::new(next_x, current_position.y + delta_y)
            }
        } else {
            let y_based_delta_x = delta_y * (endpoint_vector.x / endpoint_vector.y);
            let x_based_delta_y = delta_x * (endpoint_vector.y / endpoint_vector.x);
            let next_x = current_position.x + y_based_delta_x;
            let next_y = current_position.y + x_based_delta_y;
            if x_based_delta_y.abs() < delta_y.abs() {
                Point::new(current_position.x + delta_x, next_y)
            } else if x_based_delta_y.abs() == delta_y.abs() {
                Point::new(current_position.x + delta_x, current_position.y + delta_y)
            } else {
                Point::new(next_x, current_position.y + delta_y)
            }
        }
    }

    fn get_cell_positions_along(&self, endpoints: &Line) -> Vec<GridCell> {
        let initial_cell = GridCell::new(endpoints.p0());
        let final_cell = GridCell::new(endpoints.p1());

        if endpoints.p0() == endpoints.p1() || initial_cell.position() == final_cell.position() {
            return vec![initial_cell];
        }

        let mut cells: Vec<GridCell> = Vec::new();
        let lower_bound_x = initial_cell.position().x.min(final_cell.position().x);
        let upper_bound_x = initial_cell.position().x.max(final_cell.position().x);
        let lower_bound_y = initial_cell.position().y.min(final_cell.position().y);
        let upper_bound_y = initial_cell.position().y.max(final_cell.position().y);
        let mut current_position_along_line = endpoints.p0();
        let mut current_cell = initial_cell;
        let line_vector = endpoints.get_vector();
        let line_normal = line_vector.rotate_ccw() * (1.0 / line_vector.length());

        if matches!(self.version, GridVersion::V6_0) {
            let line_halfway = 0.5 * Vector2Df::new(line_vector.x.abs(), line_vector.y.abs());
            let line_midpoint = endpoints.p0() + 0.5 * line_vector;
            let absolute_normal = Vector2Df::new(line_normal.x.abs(), line_normal.y.abs());
            for cell_x in lower_bound_x..upper_bound_x + 1 {
                for cell_y in lower_bound_y..upper_bound_y + 1 {
                    let current_position_in_box = CELL_SIZE
                        * Vector2Df::new(f64::from(cell_x) + 0.5, f64::from(cell_y) + 0.5);
                    let next_cell_position = GridCell::new(current_position_in_box);
                    let distance_between_centers = line_midpoint - current_position_in_box;
                    let distance_from_cell_center =
                        Vector2Df::dot(absolute_normal, *next_cell_position.remainder());
                    let cell_overlap_into_hitbox = Vector2Df::dot(
                        distance_from_cell_center * Vector2Df::one(),
                        absolute_normal,
                    );
                    let normal_distance_between_centers =
                        Vector2Df::dot(line_normal, distance_between_centers);
                    let distance_from_line = (normal_distance_between_centers * line_normal.x)
                        .abs()
                        + (normal_distance_between_centers * line_normal.y).abs();
                    if line_halfway.x + next_cell_position.remainder().x
                        >= distance_between_centers.x.abs()
                        && line_halfway.y + next_cell_position.remainder().y
                            >= distance_between_centers.y.abs()
                        && cell_overlap_into_hitbox >= distance_from_line
                    {
                        cells.push(next_cell_position);
                    }
                }
            }
        } else {
            while lower_bound_x <= current_cell.position().x
                && current_cell.position().x <= upper_bound_x
                && lower_bound_y <= current_cell.position().y
                && current_cell.position().y <= upper_bound_y
            {
                current_position_along_line =
                    self.get_next_position(current_position_along_line, endpoints);
                let next_cell = GridCell::new(current_position_along_line);
                if next_cell.position() == current_cell.position() {
                    break;
                } else {
                    cells.push(current_cell);
                    current_cell = next_cell;
                }
            }
        }

        cells
    }

    pub fn get_lines_near_point(&self, point: Point) -> Vec<LineId> {
        let mut line_ids: Vec<LineId> = Vec::new();
        for i in -1..2 {
            for j in -1..2 {
                let position = CELL_SIZE * Vector2Df::new(f64::from(i), f64::from(j)) + point;
                let cell_key = GridCell::new(position).get_key();
                if let Some(cell) = self.cells.get(&cell_key) {
                    for line_id in cell.iter().rev() {
                        line_ids.push(*line_id);
                    }
                }
            }
        }
        line_ids
    }
}

#[cfg(test)]
mod tests {
    use geometry::{Line, Point};
    use serde::Deserialize;
    use std::fs;
    use vector2d::Vector2Df;

    use crate::grid::{
        Grid,
        grid_cell::{CELL_SIZE, GridCell},
    };

    #[derive(Deserialize)]
    struct GridTestCase {
        name: String,
        input: (f64, f64, f64, f64),
        expected: Vec<(i32, i32)>,
    }

    #[test]
    fn add_move_remove_lines() {
        let mut grid = Grid::new(super::GridVersion::V6_2);
        let line0 = Line::new(Point::zero(), Point::one() * CELL_SIZE);
        let line1 = Line::new(
            Point::one() * 2.0 * CELL_SIZE,
            Point::one() * 3.0 * CELL_SIZE,
        );
        let cell_key = GridCell::new(Vector2Df::zero()).get_key();

        assert!(grid.cells.is_empty(), "new grid should have no cells");

        let line0_id = grid.add_line(&line0);
        let line1_id = grid.add_line(&line0);

        assert!(
            grid.cells
                .get(&cell_key)
                .is_some_and(|cell| cell.contains(&line0_id) && cell.contains(&line1_id)),
            "first cell should have both line ids"
        );

        grid.remove_line(line1_id, &line0);

        assert!(
            grid.cells
                .get(&cell_key)
                .is_some_and(|cell| cell.contains(&line0_id) && !cell.contains(&line1_id)),
            "first cell should only have one line ids after remove"
        );

        grid.move_line(line0_id, &line0, &line1);

        assert!(
            grid.cells
                .get(&cell_key)
                .is_some_and(|cell| !cell.contains(&line0_id) && !cell.contains(&line1_id)),
            "first cell should have no line ids after move"
        );

        grid.remove_line(line0_id, &line1);

        assert!(
            !grid.cells.is_empty(),
            "grid should still have cells after removing all lines"
        );
    }

    #[test]
    fn select_near_point() {
        let mut grid = Grid::new(super::GridVersion::V6_2);
        let line0 = Line::new(Point::new(10.0, 10.0), Point::new(17.0, 10.0));
        let line1 = Line::new(Point::new(10.0, 10.0), Point::new(10.0, 17.0));
        let line2 = Line::new(Point::new(34.0, 34.0), Point::new(50.0, 36.0));
        let line0_id = grid.add_line(&line0);
        let line1_id = grid.add_line(&line1);
        let line2_id = grid.add_line(&line2);
        let lines_near_point = grid.get_lines_near_point(Point::new(-3.0, -1.0));
        assert_eq!(
            lines_near_point,
            vec![line1_id, line0_id],
            "line order should match"
        );
        let lines_near_point = grid.get_lines_near_point(Point::new(50.0, 23.0));
        assert_eq!(
            lines_near_point,
            vec![line2_id, line2_id],
            "list should contain duplicates"
        );
        let lines_near_point = grid.get_lines_near_point(Point::new(7.0, 8.0));
        assert_eq!(
            lines_near_point,
            vec![line1_id, line0_id, line1_id, line0_id],
            "cell processing order should match"
        );
        let lines_near_point = grid.get_lines_near_point(Point::new(17.0, 19.0));
        assert_eq!(
            lines_near_point,
            vec![line1_id, line0_id, line1_id, line0_id, line2_id],
            "all lines in 3x3 should be included"
        );
    }

    fn run_grid_tests(grid: Grid, data: String) {
        let test_cases: Vec<GridTestCase> =
            serde_json::from_str(&data).expect("Failed to parse JSON");

        for case in test_cases {
            let line = Line::new(
                Point::new(case.input.0, case.input.1),
                Point::new(case.input.2, case.input.3),
            );
            let grid_cells = grid.get_cell_positions_along(&line);
            assert!(
                grid_cells.len() == case.expected.len(),
                "Test '{}' failed",
                case.name
            );
            for i in 0..grid_cells.len() {
                assert!(
                    grid_cells[i].position().x == case.expected[i].0
                        && grid_cells[i].position().y == case.expected[i].1,
                    "Test '{}' failed",
                    case.name
                );
            }
        }
    }

    #[test]
    fn cell_positions_of_line_60() {
        let grid = Grid::new(super::GridVersion::V6_0);
        let data =
            fs::read_to_string("tests/grid_60_tests.json").expect("Failed to read JSON file");
        run_grid_tests(grid, data);
    }

    #[test]
    fn cell_positions_of_line_61() {
        let grid = Grid::new(super::GridVersion::V6_1);
        let data =
            fs::read_to_string("tests/grid_61_tests.json").expect("Failed to read JSON file");
        run_grid_tests(grid, data);
    }

    #[test]
    fn cell_positions_of_line_62() {
        let grid = Grid::new(super::GridVersion::V6_2);
        let data =
            fs::read_to_string("tests/grid_62_tests.json").expect("Failed to read JSON file");
        run_grid_tests(grid, data);
    }
}
