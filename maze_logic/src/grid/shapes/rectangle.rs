use crate::{
    cell::{CellType, WallState},
    grid::Shape,
};

pub struct RectangularGrid {
    grid: Vec<CellType>,
    width: usize,
    height: usize,
    walls: Vec<WallState>,
}

impl RectangularGrid {
    const NORTH: usize = 0;
    const WEST: usize = 1;
    const SOUTH: usize = 2;
    const EAST: usize = 3;

    pub fn new(width: usize, height: usize) -> Self {
        Self {
            grid: vec![CellType::default(); height * width],
            width,
            height,
            walls: vec![WallState::Solid; 4 * width * height],
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}

impl Shape for RectangularGrid {
    fn wall_states_for_cell(&self, cell_idx: usize) -> Vec<WallState> {
        vec![
            self.walls[self.wall_idx(cell_idx, Self::NORTH)],
            self.walls[self.wall_idx(cell_idx, Self::WEST)],
            self.walls[self.wall_idx(cell_idx, Self::SOUTH)],
            self.walls[self.wall_idx(cell_idx, Self::EAST)],
        ]
    }

    fn cell_type(&self, idx: usize) -> CellType {
        self.grid[idx]
    }
    fn get_row_col(&self, idx: usize) -> (usize, usize) {
        (idx / self.width, idx % self.width)
    }
    fn wall_idx(&self, cell_idx: usize, direction: usize) -> usize {
        4 * cell_idx + direction
    }

    fn set_cell_type(&mut self, cell_idx: usize, cell_type: CellType) {
        self.grid[cell_idx] = cell_type;
    }

    fn get_accessible_neighbours(&self, current: usize) -> Vec<usize> {
        let mut neighbours = Vec::new();
        let (y, x) = self.get_row_col(current);

        let north = self.wall_idx(current, Self::NORTH);
        let west = self.wall_idx(current, Self::WEST);
        let south = self.wall_idx(current, Self::SOUTH);
        let east = self.wall_idx(current, Self::EAST);

        if self.walls[north] == WallState::Open && y > 0 {
            neighbours.push(current - self.width);
        }

        if self.walls[west] == WallState::Open && x > 0 {
            neighbours.push(current - 1);
        }

        if self.walls[south] == WallState::Open && y < self.height - 1 {
            neighbours.push(current + self.width);
        }

        if self.walls[east] == WallState::Open && x < self.width - 1 {
            neighbours.push(current + 1);
        }

        neighbours
    }

    fn is_dead_end(&self, idx: usize) -> bool {
        self.walls[idx * 4..idx * 4 + 4]
            .iter()
            .filter(|&&w| w == WallState::Solid)
            .count()
            == 3
    }

    fn set_wall_state(&mut self, idx1: usize, idx2: usize, state: WallState) {
        if idx1 == idx2 {
            return;
        }
        let (y1, x1) = self.get_row_col(idx1);
        let (y2, x2) = self.get_row_col(idx2);

        if x1 == x2 {
            let (idx_north, idx_south) = if y1 > y2 { (idx1, idx2) } else { (idx2, idx1) };

            let north = self.wall_idx(idx_north, Self::NORTH);
            let south = self.wall_idx(idx_south, Self::SOUTH);

            self.walls[north] = state;
            self.walls[south] = state;
        } else if y1 == y2 {
            let (idx_west, idx_east) = if x1 > x2 { (idx1, idx2) } else { (idx2, idx1) };

            let west = self.wall_idx(idx_west, Self::WEST);
            let east = self.wall_idx(idx_east, Self::EAST);

            self.walls[west] = state;
            self.walls[east] = state;
        }
    }

    fn get_wallstate_between_neighbours(&self, idx1: usize, idx2: usize) -> WallState {
        let (y1, x1) = self.get_row_col(idx1);
        let (y2, x2) = self.get_row_col(idx2);

        if x1 == x2 {
            if y1 < y2 {
                self.walls[self.wall_idx(idx1, Self::SOUTH)]
            } else {
                self.walls[self.wall_idx(idx1, Self::NORTH)]
            }
        } else if y1 == y2 {
            if x1 < x2 {
                self.walls[self.wall_idx(idx1, Self::EAST)]
            } else {
                self.walls[self.wall_idx(idx1, Self::WEST)]
            }
        } else {
            WallState::Solid
        }
    }

    fn total_cells(&self) -> usize {
        self.width * self.height
    }

    fn get_neighbours(&self, current: usize) -> Vec<usize> {
        let mut neighbours = Vec::new();
        let (y, x) = self.get_row_col(current);

        if y > 0 {
            neighbours.push(current - self.width);
        }

        if x > 0 {
            neighbours.push(current - 1);
        }

        if y < self.height - 1 {
            neighbours.push(current + self.width);
        }

        if x < self.width - 1 {
            neighbours.push(current + 1);
        }

        neighbours
    }
}
