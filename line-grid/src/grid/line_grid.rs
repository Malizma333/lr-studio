use geometry::{Line, Point, Rectangle};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use vector2d::Vector2Df;

use crate::grid::cell_position::{CellKey, GridCell};

pub enum GridVersion {
    V6_0,
    V6_1,
    V6_2,
}

pub type LineId = u32;

pub struct Grid {
    version: GridVersion,
    cell_size: u32,
    cells: HashMap<CellKey, BTreeSet<LineId>>,
    lines: BTreeMap<LineId, Line>,
}

impl Grid {
    pub fn new(version: GridVersion, cell_size: u32) -> Grid {
        Grid {
            version,
            cell_size,
            cells: HashMap::new(),
            lines: BTreeMap::new(),
        }
    }

    fn get_next_line_id(&self) -> u32 {
        let last_id = self.lines.keys().last();
        if let Some(&id) = last_id { id + 1 } else { 0 }
    }

    fn register(&mut self, position: &GridCell, line_id: u32) {
        let cell_key = position.get_key();
        self.cells
            .entry(cell_key)
            .or_insert_with(BTreeSet::new)
            .insert(line_id);
    }

    fn unregister(&mut self, position: &GridCell, line_id: u32) {
        let cell_key = position.get_key();
        if let Some(cell) = self.cells.get_mut(&cell_key) {
            cell.remove(&line_id);
        }
    }

    pub fn add_line(&mut self, line: Line) -> u32 {
        let id = self.get_next_line_id();
        let cell_positions = self.get_cell_positions_along(&line);
        self.lines.insert(id, line);
        for position in cell_positions {
            self.register(&position, id);
        }
        id
    }

    pub fn remove_line(&mut self, id: u32) {
        let optional_line = self.lines.remove(&id);
        if let Some(line) = optional_line {
            let cell_positions = self.get_cell_positions_along(&line);
            for position in cell_positions {
                self.unregister(&position, id);
            }
        }
    }

    pub fn move_line(&mut self, id: u32, new_line: Line) {
        let optional_line = self.lines.remove(&id);
        if let Some(line) = optional_line {
            let cell_positions = self.get_cell_positions_along(&line);
            for position in cell_positions {
                self.unregister(&position, id);
            }

            let new_cell_positions = self.get_cell_positions_along(&new_line);
            for position in new_cell_positions {
                self.register(&position, id);
            }

            self.lines.insert(id, new_line);
        }
    }

    fn get_next_position(&self, current_position: Point, line: &Line) -> Point {
        let current_cell = GridCell::new(current_position, self.cell_size);
        let line_vector = line.1 - line.0;

        let mut delta_x = if line_vector.x() > 0.0 {
            f64::from(self.cell_size) - current_cell.remainder().x()
        } else {
            -1.0 - current_cell.remainder().x()
        };

        let mut delta_y = if line_vector.y() > 0.0 {
            f64::from(self.cell_size) - current_cell.remainder().y()
        } else {
            -1.0 - current_cell.remainder().y()
        };

        if matches!(self.version, GridVersion::V6_2) {
            if current_cell.position().x() < 0 {
                delta_x = if line_vector.x() > 0.0 {
                    f64::from(self.cell_size) + current_cell.remainder().x()
                } else {
                    -(f64::from(self.cell_size) + current_cell.remainder().x())
                }
            }
            if current_cell.position().y() < 0 {
                delta_y = if line_vector.y() > 0.0 {
                    f64::from(self.cell_size) + current_cell.remainder().y()
                } else {
                    -(f64::from(self.cell_size) + current_cell.remainder().y())
                }
            }
        }

        if line_vector.x() == 0.0 {
            Point::new(current_position.x(), current_position.y() + delta_y)
        } else if line_vector.y() == 0.0 {
            Point::new(current_position.x() + delta_x, current_position.y())
        } else if matches!(self.version, GridVersion::V6_1) {
            let slope = line_vector.y() / line_vector.x();
            let y_intercept = line.0.y() - slope * line.0.x();
            let next_x = ((current_position.y() + delta_y - y_intercept) / slope).round();
            let next_y = (slope * (current_position.x() + delta_x) + y_intercept).round();
            if (next_y - current_position.y()).abs() < delta_y.abs() {
                Point::new(current_position.x() + delta_x, next_y)
            } else if (next_y - current_position.y()).abs() == delta_y.abs() {
                Point::new(
                    current_position.x() + delta_x,
                    current_position.y() + delta_y,
                )
            } else {
                Point::new(next_x, current_position.y() + delta_y)
            }
        } else {
            let y_based_delta_x = delta_y * (line_vector.x() / line_vector.y());
            let x_based_delta_y = delta_x * (line_vector.y() / line_vector.x());
            let next_x = current_position.x() + y_based_delta_x;
            let next_y = current_position.y() + x_based_delta_y;
            if x_based_delta_y.abs() < delta_y.abs() {
                Point::new(current_position.x() + delta_x, next_y)
            } else if x_based_delta_y.abs() == delta_y.abs() {
                Point::new(
                    current_position.x() + delta_x,
                    current_position.y() + delta_y,
                )
            } else {
                Point::new(next_x, current_position.y() + delta_y)
            }
        }
    }

    fn get_cell_positions_along(&self, line: &Line) -> Vec<GridCell> {
        let initial_cell = GridCell::new(line.0, self.cell_size);
        let final_cell = GridCell::new(line.1, self.cell_size);

        if line.0 == line.1 || initial_cell.position() == final_cell.position() {
            return vec![initial_cell.clone()];
        }

        let mut cells: Vec<GridCell> = Vec::new();
        let lower_bound_x = initial_cell.position().x().min(final_cell.position().x());
        let upper_bound_x = initial_cell.position().x().max(final_cell.position().x());
        let lower_bound_y = initial_cell.position().y().min(final_cell.position().y());
        let upper_bound_y = initial_cell.position().y().max(final_cell.position().y());
        let mut current_position_along_line = line.0;
        let mut current_cell = initial_cell;
        let line_vector = line.1 - line.0;
        let line_normal = line_vector.rotate_ccw() * (1.0 / line_vector.length());

        if matches!(self.version, GridVersion::V6_0) {
            let line_halfway = 0.5 * Vector2Df::new(line_vector.x().abs(), line_vector.y().abs());
            let line_midpoint = line.0 + 0.5 * line_vector;
            let absolute_normal = Vector2Df::new(line_normal.x().abs(), line_normal.y().abs());
            for cell_x in lower_bound_x..upper_bound_x + 1 {
                for cell_y in lower_bound_y..upper_bound_y + 1 {
                    let current_position_in_box = f64::from(self.cell_size)
                        * Vector2Df::new(f64::from(cell_x) + 0.5, f64::from(cell_y) + 0.5);
                    let next_cell_position = GridCell::new(current_position_in_box, self.cell_size);
                    let distance_between_centers = line_midpoint - current_position_in_box;
                    let distance_from_cell_center =
                        Vector2Df::dot(absolute_normal, *next_cell_position.remainder());
                    let cell_overlap_into_hitbox = Vector2Df::dot(
                        distance_from_cell_center * Vector2Df::one(),
                        absolute_normal,
                    );
                    let normal_distance_between_centers =
                        Vector2Df::dot(line_normal, distance_between_centers);
                    let distance_from_line = (normal_distance_between_centers * line_normal.x())
                        .abs()
                        + (normal_distance_between_centers * line_normal.y()).abs();
                    if line_halfway.x() + next_cell_position.remainder().x()
                        >= distance_between_centers.x().abs()
                        && line_halfway.y() + next_cell_position.remainder().y()
                            >= distance_between_centers.y().abs()
                        && cell_overlap_into_hitbox >= distance_from_line
                    {
                        cells.push(next_cell_position.clone());
                    }
                }
            }
        } else {
            while lower_bound_x <= current_cell.position().x()
                && current_cell.position().x() <= upper_bound_x
                && lower_bound_y <= current_cell.position().y()
                && current_cell.position().y() <= upper_bound_y
            {
                current_position_along_line =
                    self.get_next_position(current_position_along_line, line);
                let next_cell = GridCell::new(current_position_along_line, self.cell_size);
                if next_cell.position() == current_cell.position() {
                    break;
                } else {
                    cells.push(current_cell.clone());
                    current_cell = next_cell;
                }
            }
        }

        cells
    }

    /** Finds all of the lines that lie within a rectangular region (inclusive) and returns their ids */
    pub fn select_lines_in_rect(&self, rectangle: Rectangle) -> Vec<u32> {
        let mut regional_lines: HashSet<u32> = HashSet::new();
        let lower_cell = GridCell::new(rectangle.bottom_left(), self.cell_size);
        let upper_cell = GridCell::new(rectangle.top_right(), self.cell_size);

        for cell_x in lower_cell.position().x()..upper_cell.position().x() + 1 {
            for cell_y in lower_cell.position().y()..upper_cell.position().y() + 1 {
                let key = GridCell::new(
                    Point::new(f64::from(cell_x), f64::from(cell_y)),
                    self.cell_size,
                )
                .get_key();
                if let Some(cell) = self.cells.get(&key) {
                    regional_lines.extend(cell.iter());
                }
            }
        }

        let mut lines_included: Vec<u32> = Vec::new();

        for id in regional_lines {
            // TODO: make sure that line should exist in self.lines (otherwise something went wrong)
            if let Some(line) = self.lines.get(&id) {
                if rectangle.contains(line) {
                    lines_included.push(id);
                }
            }
        }

        lines_included
    }

    pub fn select_lines_in_radius(&self, center: Point, radius: f64) -> Vec<u32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::grid::{Grid, cell_position::GridCell};
    use geometry::{Line, Point, Rectangle};

    #[test]
    fn line_id() {
        let cell_size = 1;
        let mut grid = Grid::new(super::GridVersion::V6_2, cell_size);

        grid.lines.insert(0, Line(Point::zero(), Point::zero()));

        assert_eq!(grid.get_next_line_id(), 1);

        grid.lines.insert(6, Line(Point::zero(), Point::zero()));
        grid.lines.insert(3, Line(Point::zero(), Point::zero()));

        assert_eq!(grid.get_next_line_id(), 7);

        grid.lines.remove(&0);

        assert_eq!(grid.get_next_line_id(), 7);

        grid.lines.remove(&6);

        assert_eq!(grid.get_next_line_id(), 4);
    }

    #[test]
    fn register_unregister() {
        let cell_size = 1;
        let line_zero = 0;
        let line_one = 1;
        let world_position = Point::new(0.5, 0.5);
        let mut grid = Grid::new(super::GridVersion::V6_2, cell_size);
        let position = GridCell::new(world_position, cell_size);
        let key = position.get_key();

        grid.register(&position, line_zero);

        assert!(grid.cells.get(&key).is_some_and(|x| x.contains(&line_zero)));

        grid.unregister(&position, line_zero);

        assert!(
            grid.cells
                .get(&key)
                .is_some_and(|x| !x.contains(&line_zero))
        );

        grid.register(&position, line_one);

        assert!(
            grid.cells
                .get(&key)
                .is_some_and(|x| !x.contains(&line_zero))
        );

        grid.register(&position, line_zero);

        assert!(grid.cells.get(&key).is_some_and(|x| x.contains(&line_zero)));
    }

    #[test]
    fn cell_positions_of_line_60() {
        todo!()
    }

    #[test]
    fn cell_positions_of_line_61() {
        todo!()
    }

    #[test]
    fn cell_positions_of_line_62() {
        todo!()
    }

    #[test]
    fn add_line() {
        let mut grid = Grid::new(super::GridVersion::V6_2, 1);

        assert!(grid.cells.is_empty(), "new grid should have no cells");
        assert!(grid.lines.is_empty(), "new grid should have no lines");

        grid.add_line(Line(Point::zero(), Point::one()));

        assert!(!grid.cells.is_empty(), "adding line should create cell");
        assert!(
            grid.lines.contains_key(&0),
            "first line added should have id 0"
        );
        assert!(
            grid.lines
                .get(&0)
                .is_some_and(|x| x.0 == Point::zero() && x.1 == Point::one()),
            "first line added should match input"
        );

        grid.add_line(Line(Point::one(), Point::zero()));
        grid.add_line(Line(Point::one() * -1.0, Point::zero()));

        assert!(
            grid.lines.contains_key(&2),
            "id count should reach 2 with three new lines added"
        );
        assert!(
            grid.lines
                .get(&2)
                .is_some_and(|x| x.0 == Point::one() * -1.0 && x.1 == Point::zero()),
            "third line should match input"
        );
    }

    #[test]
    fn remove_line() {
        let mut grid = Grid::new(super::GridVersion::V6_2, 1);
        grid.add_line(Line(Point::zero(), Point::one()));
        grid.add_line(Line(Point::one(), Point::zero()));
        grid.remove_line(0);
        assert!(
            grid.lines.contains_key(&1),
            "removing line with id 0 should keep line with id 1 intact"
        );
        grid.remove_line(1);
        assert!(
            !grid.cells.is_empty(),
            "grid should still have cells after removing all lines"
        );
        assert!(
            grid.lines.is_empty(),
            "grid should not have any lines after removing all lines"
        );
    }

    #[test]
    fn move_line() {
        let mut grid = Grid::new(super::GridVersion::V6_2, 1);
        grid.add_line(Line(Point::one(), Point::zero()));
        assert!(
            grid.lines
                .get(&0)
                .is_some_and(|x| x.0 == Point::one() && x.1 == Point::zero()),
            "line endpoints should match input"
        );
        grid.move_line(0, Line(Point::one(), Point::one()));
        assert!(
            grid.lines
                .get(&0)
                .is_some_and(|x| x.0 == Point::one() && x.1 == Point::one()),
            "same line endpoints should match new input after moving"
        );
    }

    #[test]
    fn select_rect() {
        let mut grid = Grid::new(super::GridVersion::V6_2, 1);
        grid.add_line(Line(Point::one(), Point::zero()));
        grid.add_line(Line(Point::one(), Point::one() * 2.0));
        grid.add_line(Line(Point::one() * 0.5, Point::one() * 2.0));
        let lines =
            grid.select_lines_in_rect(Rectangle::new(Point::one() * -1.0, Point::one() * 5.0));
        assert!(lines.len() == 3);
        let lines = grid.select_lines_in_rect(Rectangle::new(Point::one(), Point::zero()));
        assert!(lines.len() == 3);
        let lines = grid.select_lines_in_rect(Rectangle::new(Point::one() * 0.5, Point::zero()));
        assert!(lines.len() == 2);
        let lines = grid.select_lines_in_rect(Rectangle::new(Point::one() * 0.9, Point::zero()));
        assert!(lines.len() == 2);
        let lines =
            grid.select_lines_in_rect(Rectangle::new(Point::one() * 0.4, Point::one() * 0.2));
        assert!(lines.len() == 1);
        let lines = grid.select_lines_in_rect(Rectangle::new(Point::zero(), Point::zero()));
        assert!(lines.len() == 1);
        let lines =
            grid.select_lines_in_rect(Rectangle::new(Point::one() * -1.0, Point::one() * -0.5));
        assert!(lines.len() == 0);
    }

    #[test]
    fn select_circle() {
        todo!()
    }
}
