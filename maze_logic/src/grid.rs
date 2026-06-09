use crate::cell::{CellType, WallState};

pub mod shapes;
pub trait Shape {
    fn get_accessible_neighbours(&self, current: usize) -> Vec<usize>;
    fn is_dead_end(&self, idx: usize) -> bool;
    fn get_wallstate_between_neighbours(&self, idx1: usize, idx2: usize) -> WallState;
    fn set_cell_type(&mut self, cell_idx: usize, cell_type: CellType);
    fn total_cells(&self) -> usize;
    fn get_neighbours(&self, current: usize) -> Vec<usize>;

    fn wall_idx(&self, cell_idx: usize, direction: usize) -> usize;
    fn get_row_col(&self, idx: usize) -> (usize, usize);
    fn set_wall_state(&mut self, idx1: usize, idx2: usize, state: WallState);
}
