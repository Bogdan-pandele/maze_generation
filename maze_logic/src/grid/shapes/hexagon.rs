use crate::{
    cell::{CellType, WallState},
    grid::Shape,
};

#[derive(Debug)]
pub struct HexagonalGrid {
    grid: Vec<CellType>,
    width: usize,
    height: usize,
    walls: Vec<WallState>,
}

impl HexagonalGrid {
    const TOP_RIGHT: usize = 0;
    const RIGHT: usize = 1;
    const BOTTOM_RIGHT: usize = 2;
    const BOTTOM_LEFT: usize = 3;
    const LEFT: usize = 4;
    const TOP_LEFT: usize = 5;

    pub fn new(width: usize, height: usize) -> Self {
        Self {
            grid: vec![CellType::default(); width * height],
            width,
            height,
            walls: vec![WallState::Solid; 6 * width * height],
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}

impl Shape for HexagonalGrid {
    fn wall_states_for_cell(&self, cell_idx: usize) -> Vec<WallState> {
        vec![
            self.walls[self.wall_idx(cell_idx, Self::TOP_RIGHT)],
            self.walls[self.wall_idx(cell_idx, Self::RIGHT)],
            self.walls[self.wall_idx(cell_idx, Self::BOTTOM_RIGHT)],
            self.walls[self.wall_idx(cell_idx, Self::BOTTOM_LEFT)],
            self.walls[self.wall_idx(cell_idx, Self::LEFT)],
            self.walls[self.wall_idx(cell_idx, Self::TOP_LEFT)],
        ]
    }
    fn cell_type(&self, idx: usize) -> CellType {
        self.grid[idx]
    }
    fn get_row_col(&self, idx: usize) -> (usize, usize) {
        (idx / self.width, idx % self.width)
    }
    fn wall_idx(&self, cell_idx: usize, direction: usize) -> usize {
        6 * cell_idx + direction
    }

    fn set_cell_type(&mut self, cell_idx: usize, cell_type: CellType) {
        self.grid[cell_idx] = cell_type;
    }

    fn get_accessible_neighbours(&self, current: usize) -> Vec<usize> {
        let mut neighbours = Vec::new();
        let (row, col) = self.get_row_col(current);

        let top_right = self.wall_idx(current, Self::TOP_RIGHT);
        let right = self.wall_idx(current, Self::RIGHT);
        let bottom_right = self.wall_idx(current, Self::BOTTOM_RIGHT);
        let bottom_left = self.wall_idx(current, Self::BOTTOM_LEFT);
        let left = self.wall_idx(current, Self::LEFT);
        let top_left = self.wall_idx(current, Self::TOP_LEFT);

        if col > 0 && self.walls[left] == WallState::Open {
            neighbours.push(current - 1);
        }

        if col < self.width - 1 && self.walls[right] == WallState::Open {
            neighbours.push(current + 1);
        }

        let is_odd_row = row % 2 != 0;

        if is_odd_row {
            if row > 0 && self.walls[top_left] == WallState::Open {
                neighbours.push(current - self.width);
            }
            if row > 0 && col < self.width - 1 && self.walls[top_right] == WallState::Open {
                neighbours.push(current - self.width + 1);
            }
            if row < self.height - 1 && self.walls[bottom_left] == WallState::Open {
                neighbours.push(current + self.width);
            }
            if row < self.height - 1
                && col < self.width - 1
                && self.walls[bottom_right] == WallState::Open
            {
                neighbours.push(current + self.width + 1);
            }
        } else {
            if row > 0 && col > 0 && self.walls[top_left] == WallState::Open {
                neighbours.push(current - self.width - 1);
            }
            if row > 0 && self.walls[top_right] == WallState::Open {
                neighbours.push(current - self.width);
            }
            if row < self.height - 1 && col > 0 && self.walls[bottom_left] == WallState::Open {
                neighbours.push(current + self.width - 1);
            }
            if row < self.height - 1 && self.walls[bottom_right] == WallState::Open {
                neighbours.push(current + self.width);
            }
        }

        neighbours
    }

    fn is_dead_end(&self, idx: usize) -> bool {
        self.walls[idx * 6..idx * 6 + 6]
            .iter()
            .filter(|&&w| w == WallState::Solid)
            .count()
            == 5
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
            let mut set_wallstate =
                |idx1: usize, direction1: usize, idx2: usize, direction2: usize| {
                    let (wall1, wall2) = (
                        self.wall_idx(idx1, direction1),
                        self.wall_idx(idx2, direction2),
                    );

                    self.walls[wall1] = state;
                    self.walls[wall2] = state;
                };

            let is_odd_row1 = row1 % 2 != 0;

            if is_odd_row1 {
                if row2 > row1 && col2 == col1 {
                    set_wallstate(idx1, Self::BOTTOM_LEFT, idx2, Self::TOP_RIGHT);
                } else if row2 > row1 && col2 > col1 {
                    set_wallstate(idx1, Self::BOTTOM_RIGHT, idx2, Self::TOP_LEFT);
                } else if row2 < row1 && col2 == col1 {
                    set_wallstate(idx1, Self::TOP_LEFT, idx2, Self::BOTTOM_RIGHT);
                } else if row2 < row1 && col2 > col1 {
                    set_wallstate(idx1, Self::TOP_RIGHT, idx2, Self::BOTTOM_LEFT);
                }
            } else {
                if row2 > row1 && col2 == col1 {
                    set_wallstate(idx1, Self::BOTTOM_RIGHT, idx2, Self::TOP_LEFT);
                } else if row2 > row1 && col2 < col1 {
                    set_wallstate(idx1, Self::BOTTOM_LEFT, idx2, Self::TOP_RIGHT);
                } else if row2 < row1 && col2 == col1 {
                    set_wallstate(idx1, Self::TOP_RIGHT, idx2, Self::BOTTOM_LEFT);
                } else if row2 < row1 && col2 < col1 {
                    set_wallstate(idx1, Self::TOP_LEFT, idx2, Self::BOTTOM_RIGHT);
                }
            }
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
            let is_odd_row1 = row1 % 2 != 0;

            if is_odd_row1 {
                if row2 > row1 && col2 == col1 {
                    self.walls[self.wall_idx(idx1, Self::BOTTOM_LEFT)]
                } else if row2 > row1 && col2 > col1 {
                    self.walls[self.wall_idx(idx1, Self::BOTTOM_RIGHT)]
                } else if row2 < row1 && col2 == col1 {
                    self.walls[self.wall_idx(idx1, Self::TOP_LEFT)]
                } else if row2 < row1 && col2 > col1 {
                    self.walls[self.wall_idx(idx1, Self::TOP_RIGHT)]
                } else {
                    WallState::Solid
                }
            } else {
                if row2 > row1 && col2 == col1 {
                    self.walls[self.wall_idx(idx1, Self::BOTTOM_RIGHT)]
                } else if row2 > row1 && col2 < col1 {
                    self.walls[self.wall_idx(idx1, Self::BOTTOM_LEFT)]
                } else if row2 < row1 && col2 == col1 {
                    self.walls[self.wall_idx(idx1, Self::TOP_RIGHT)]
                } else if row2 < row1 && col2 < col1 {
                    self.walls[self.wall_idx(idx1, Self::TOP_LEFT)]
                } else {
                    WallState::Solid
                }
            }
        }
    }

    fn total_cells(&self) -> usize {
        self.width * self.height
    }

    fn get_neighbours(&self, current: usize) -> Vec<usize> {
        let mut neighbours = Vec::new();
        let (row, col) = self.get_row_col(current);
        let is_odd = row % 2 != 0;

        if col > 0 {
            neighbours.push(current - 1);
        }
        if col < self.width - 1 {
            neighbours.push(current + 1);
        }

        if is_odd {
            if row > 0 {
                neighbours.push(current - self.width);
            }
            if row > 0 && col < self.width - 1 {
                neighbours.push(current - self.width + 1);
            }
            if row < self.height - 1 {
                neighbours.push(current + self.width);
            }
            if row < self.height - 1 && col < self.width - 1 {
                neighbours.push(current + self.width + 1);
            }
        } else {
            if row > 0 && col > 0 {
                neighbours.push(current - self.width - 1);
            }
            if row > 0 {
                neighbours.push(current - self.width);
            }
            if row < self.height - 1 && col > 0 {
                neighbours.push(current + self.width - 1);
            }
            if row < self.height - 1 {
                neighbours.push(current + self.width);
            }
        }

        neighbours
    }
}
