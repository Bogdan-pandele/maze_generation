use std::collections::{HashSet, VecDeque};

use crate::{
    cell::{CellType, WallState},
    grid::Shape,
};

#[derive(Debug)]
pub struct Maze<S: Shape> {
    shape: S,
    start: usize,
    end: usize,
}

impl<S: Shape> Maze<S> {
    pub fn new(shape: S) -> Self {
        Self {
            shape,
            start: 0,
            end: 0,
        }
    }

    pub fn wall_states_for_cell(&self, idx: usize) -> Vec<WallState> {
        self.shape.wall_states_for_cell(idx)
    }
    pub fn cell_type(&self, idx: usize) -> CellType {
        self.shape.cell_type(idx)
    }

    pub fn start(&self) -> usize {
        self.start
    }

    pub fn end(&self) -> usize {
        self.end
    }

    pub fn shape(&self) -> &S {
        &self.shape
    }

    pub fn set_start(&mut self, start: usize) {
        self.start = start;
    }

    pub fn set_end(&mut self, end: usize) {
        self.end = end;
    }

    pub fn size(&self) -> usize {
        self.shape.total_cells()
    }

    pub fn get_accessible_neighbours(&self, idx: usize) -> Vec<usize> {
        self.shape.get_accessible_neighbours(idx)
    }

    pub fn get_neighbours(&self, idx: usize) -> Vec<usize> {
        self.shape.get_neighbours(idx)
    }

    pub fn is_dead_end(&self, idx: usize) -> bool {
        self.shape.is_dead_end(idx)
    }

    pub fn get_wallstate_bewteen_neighbours(&self, idx1: usize, idx2: usize) -> WallState {
        self.shape.get_wallstate_between_neighbours(idx1, idx2)
    }

    pub fn set_cell_type(&mut self, cell_idx: usize, cell_type: CellType) {
        self.shape.set_cell_type(cell_idx, cell_type);
    }

    pub fn get_accessible_zone(&self, current_key: u8) -> HashSet<usize> {
        let mut accessible_zone = HashSet::new();
        let mut visited = vec![false; self.shape.total_cells()];

        let mut queue = VecDeque::new();

        queue.push_back(self.start);
        visited[self.start] = true;

        while let Some(current) = queue.pop_front() {
            accessible_zone.insert(current);

            for neighbour in self.shape.get_neighbours(current) {
                if visited[neighbour] {
                    continue;
                }

                let wall_state = self.get_wallstate_bewteen_neighbours(current, neighbour);
                let can_pass = match wall_state {
                    WallState::Open => true,
                    WallState::Door(id) => current_key > id,
                    WallState::Solid => false,
                };

                if can_pass {
                    queue.push_back(neighbour);
                    visited[neighbour] = true;
                }
            }
        }
        accessible_zone
    }

    pub fn remove_wall(&mut self, idx1: usize, idx2: usize) {
        self.shape.set_wall_state(idx1, idx2, WallState::Open);
    }

    pub fn place_door(&mut self, idx1: usize, idx2: usize, door_id: u8) {
        self.shape
            .set_wall_state(idx1, idx2, WallState::Door(door_id));
    }
}
