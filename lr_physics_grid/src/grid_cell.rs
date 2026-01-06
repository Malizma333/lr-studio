use geometry::Point;
use vector2d::{Vector2Df, Vector2Di};

pub(super) const CELL_SIZE: f64 = 14.0;
pub(super) type CellKey = i32;

pub(super) struct GridCell {
    position: Vector2Di,
    remainder: Vector2Df,
}

impl GridCell {
    pub fn new(world_position: Point) -> GridCell {
        let scaled_position = world_position / f64::from(CELL_SIZE);
        let position = Vector2Di::new(
            scaled_position.x().floor() as i32,
            scaled_position.y().floor() as i32,
        );
        let remainder = world_position - f64::from(CELL_SIZE) * Vector2Df::from(position);
        GridCell {
            position,
            remainder,
        }
    }

    pub fn position(&self) -> &Vector2Di {
        &self.position
    }

    pub fn remainder(&self) -> &Vector2Df {
        &self.remainder
    }

    pub fn get_key(&self) -> CellKey {
        let x_comp = if self.position.x() >= 0 {
            2 * self.position.x()
        } else {
            -2 * self.position.x() - 1
        };

        let y_comp = if self.position.y() >= 0 {
            2 * self.position.y()
        } else {
            -2 * self.position.y() - 1
        };

        let hash = if x_comp >= y_comp {
            x_comp * x_comp + x_comp + y_comp
        } else {
            y_comp * y_comp + x_comp
        };

        if hash % 2 == 1 {
            (-(hash - 1) / 2) - 1
        } else {
            hash / 2 + 1
        }
    }
}

#[cfg(test)]
mod tests {
    use geometry::Point;
    use std::collections::HashMap;

    use crate::grid_cell::{CELL_SIZE, GridCell};

    #[test]
    fn unique_hash() {
        let mut seen: HashMap<i32, (i32, i32)> = HashMap::new();

        for i in -10..11 {
            for j in -10..11 {
                let key =
                    GridCell::new(CELL_SIZE * Point::new(f64::from(i), f64::from(j))).get_key();
                assert!(!seen.contains_key(&key));
                seen.insert(key, (i, j));
            }
        }
    }
}
