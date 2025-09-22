mod grid_cell;
mod line_grid;

const CELL_SIZE: f64 = 14.0;

type CellKey = i32;
type LineId = u32;

pub enum GridVersion {
    V6_0,
    V6_1,
    V6_2,
}
