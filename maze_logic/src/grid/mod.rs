use crate::{cell::CellType, grid::shapes::rectangular::WallState};

pub mod shapes;
pub trait Shape {
    fn get_accessible_neighbours(&self, current: usize) -> Vec<usize>;
    fn is_dead_end(&self, idx: usize) -> bool;
    fn remove_wall(&mut self, idx1: usize, idx2: usize);
    fn get_wallstate_bewteen_neighbours(&self, idx1: usize, idx2: usize) -> WallState;
    fn set_cell_type(&mut self, cell_idx: usize, cell_type: CellType);
    fn size(&self) -> usize;
    fn get_neighbours(&self, current: usize) -> Vec<usize>;
    fn place_door(&mut self, idx1: usize, idx2: usize, door_id: u8);
    fn format_grid(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        start: usize,
        end: usize,
    ) -> std::fmt::Result;
}
