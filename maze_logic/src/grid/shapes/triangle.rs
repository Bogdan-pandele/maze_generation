use crate::{
    cell::{CellType, WallState},
    grid::Shape,
};

#[derive(Debug)]
pub struct TriangularGrid {
    grid: Vec<CellType>,
    width: usize,
    height: usize,
    walls: Vec<WallState>,
}

impl TriangularGrid {
    const LEFT: usize = 0;
    const RIGHT: usize = 1;
    const TOP_OR_BOTTOM: usize = 2;

    pub fn new(width: usize, height: usize) -> Self {
        Self {
            grid: vec![CellType::default(); width * height],
            width,
            height,
            walls: vec![WallState::Solid; 3 * width * height],
        }
    }

    fn is_upside_down(&self, idx: usize) -> bool {
        let (row, col) = self.get_row_col(idx);

        if (row % 2 == 0 && col % 2 != 0) || (row % 2 != 0 && col % 2 == 0) {
            return true;
        }

        false
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}

impl Shape for TriangularGrid {
    fn wall_states_for_cell(&self, cell_idx: usize) -> Vec<WallState> {
        vec![
            self.walls[self.wall_idx(cell_idx, Self::LEFT)],
            self.walls[self.wall_idx(cell_idx, Self::RIGHT)],
            self.walls[self.wall_idx(cell_idx, Self::TOP_OR_BOTTOM)],
        ]
    }

    fn cell_type(&self, idx: usize) -> CellType {
        self.grid[idx]
    }

    fn get_row_col(&self, idx: usize) -> (usize, usize) {
        (idx / self.width, idx % self.width)
    }

    fn wall_idx(&self, cell_idx: usize, direction: usize) -> usize {
        3 * cell_idx + direction
    }

    fn set_cell_type(&mut self, cell_idx: usize, cell_type: CellType) {
        self.grid[cell_idx] = cell_type;
    }

    fn get_accessible_neighbours(&self, current: usize) -> Vec<usize> {
        let mut neighbours = Vec::new();
        let (row, col) = self.get_row_col(current);

        let left_wall_idx = self.wall_idx(current, Self::LEFT);
        let right_wall_idx = self.wall_idx(current, Self::RIGHT);
        let top_bottom_wall_idx = self.wall_idx(current, Self::TOP_OR_BOTTOM);

        if self.walls[left_wall_idx] == WallState::Open && col > 0 {
            neighbours.push(current - 1);
        }

        if self.walls[right_wall_idx] == WallState::Open && col < self.width - 1 {
            neighbours.push(current + 1);
        }

        if self.walls[top_bottom_wall_idx] == WallState::Open {
            if self.is_upside_down(current) {
                if row > 0 {
                    neighbours.push(current - self.width);
                }
            } else {
                if row < self.height - 1 {
                    neighbours.push(current + self.width);
                }
            }
        }
        neighbours
    }

    fn is_dead_end(&self, idx: usize) -> bool {
        self.walls[3 * idx..3 * idx + 3]
            .iter()
            .filter(|&&w| w == WallState::Open)
            .count()
            == 2
    }

    fn set_wall_state(&mut self, idx1: usize, idx2: usize, state: WallState) {
        if idx1 == idx2 {
            return;
        }

        let (row1, col1) = self.get_row_col(idx1);
        let (row2, col2) = self.get_row_col(idx2);

        if row1 == row2 {
            let (left_wall_idx, right_wall_idx) = if col1 < col2 {
                (
                    self.wall_idx(idx2, Self::LEFT),
                    self.wall_idx(idx1, Self::RIGHT),
                )
            } else {
                (
                    self.wall_idx(idx1, Self::LEFT),
                    self.wall_idx(idx2, Self::RIGHT),
                )
            };

            self.walls[left_wall_idx] = state;
            self.walls[right_wall_idx] = state;
        } else {
            let top_or_bottom1 = self.wall_idx(idx1, Self::TOP_OR_BOTTOM);
            let top_or_bottom2 = self.wall_idx(idx2, Self::TOP_OR_BOTTOM);

            self.walls[top_or_bottom1] = state;
            self.walls[top_or_bottom2] = state;
        }
    }

    fn get_wallstate_between_neighbours(&self, idx1: usize, idx2: usize) -> WallState {
        let (row1, col1) = self.get_row_col(idx1);
        let (row2, col2) = self.get_row_col(idx2);

        if row1 == row2 {
            if col1 < col2 {
                self.walls[self.wall_idx(idx1, Self::RIGHT)]
            } else {
                self.walls[self.wall_idx(idx1, Self::LEFT)]
            }
        } else {
            self.walls[self.wall_idx(idx1, Self::TOP_OR_BOTTOM)]
        }
    }

    fn total_cells(&self) -> usize {
        self.width * self.height
    }

    fn get_neighbours(&self, current: usize) -> Vec<usize> {
        let mut neighbours = Vec::new();

        let (row, col) = self.get_row_col(current);

        if col > 0 {
            neighbours.push(current - 1);
        }

        if col < self.width - 1 {
            neighbours.push(current + 1);
        }

        if self.is_upside_down(current) {
            if row > 0 {
                neighbours.push(current - self.width);
            }
        } else {
            if row < self.height - 1 {
                neighbours.push(current + self.width);
            }
        }

        neighbours
    }
}
