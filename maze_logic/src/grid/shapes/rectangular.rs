use crate::{cell::CellType, grid::Shape};

#[derive(Clone, PartialEq, Eq, Copy)]
pub enum WallState {
    Solid,
    Door(u8),
    Open,
}

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

    fn wall_idx(&self, cell_idx: usize, direction: usize) -> usize {
        4 * cell_idx + direction
    }
}

impl Shape for RectangularGrid {
    fn set_cell_type(&mut self, cell_idx: usize, cell_type: CellType) {
        self.grid[cell_idx] = cell_type;
    }

    fn get_accessible_neighbours(&self, current: usize) -> Vec<usize> {
        let mut neighbours = Vec::new();
        let x = current % self.width;
        let y = current / self.width;

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

    fn is_dead_end(&self, current: usize) -> bool {
        self.walls[current * 4..current * 4 + 4]
            .iter()
            .filter(|&&w| w == WallState::Solid)
            .count()
            == 3
    }

    fn remove_wall(&mut self, idx1: usize, idx2: usize) {
        if idx1 == idx2 {
            return;
        }

        let x1 = idx1 % self.width;
        let y1 = idx1 / self.width;
        let x2 = idx2 % self.width;
        let y2 = idx2 / self.width;

        if x1 == x2 {
            let (idx_north, idx_south) = if y1 > y2 { (idx1, idx2) } else { (idx2, idx1) };

            let north = self.wall_idx(idx_north, Self::NORTH);
            let south = self.wall_idx(idx_south, Self::SOUTH);

            self.walls[north] = WallState::Open;
            self.walls[south] = WallState::Open;
        } else if y1 == y2 {
            let (idx_west, idx_east) = if x1 > x2 { (idx1, idx2) } else { (idx2, idx1) };

            let west = self.wall_idx(idx_west, Self::WEST);
            let east = self.wall_idx(idx_east, Self::EAST);

            self.walls[west] = WallState::Open;
            self.walls[east] = WallState::Open;
        }
    }

    fn get_wallstate_bewteen_neighbours(&self, idx1: usize, idx2: usize) -> WallState {
        let x1 = idx1 % self.width;
        let y1 = idx1 / self.width;
        let x2 = idx2 % self.width;
        let y2 = idx2 / self.width;

        if x1 == x2 {
            return if y1 < y2 {
                self.walls[self.wall_idx(idx1, Self::SOUTH)]
            } else {
                self.walls[self.wall_idx(idx1, Self::NORTH)]
            };
        } else if y1 == y2 {
            return if x1 < x2 {
                self.walls[self.wall_idx(idx1, Self::EAST)]
            } else {
                self.walls[self.wall_idx(idx1, Self::WEST)]
            };
        } else {
            WallState::Solid
        }
    }

    fn size(&self) -> usize {
        self.width * self.height
    }

    fn get_neighbours(&self, current: usize) -> Vec<usize> {
        let mut neighbours = Vec::new();
        let x = current % self.width;
        let y = current / self.width;

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

    fn place_door(&mut self, idx1: usize, idx2: usize, door_id: u8) {
        let x1 = idx1 % self.width;
        let y1 = idx1 / self.width;
        let x2 = idx2 % self.width;
        let y2 = idx2 / self.width;

        if x1 == x2 {
            let (idx_north, idx_south) = if y1 > y2 { (idx1, idx2) } else { (idx2, idx1) };

            let north = self.wall_idx(idx_north, Self::NORTH);
            let south = self.wall_idx(idx_south, Self::SOUTH);

            self.walls[north] = WallState::Door(door_id);
            self.walls[south] = WallState::Door(door_id);
        } else if y1 == y2 {
            let (idx_west, idx_east) = if x1 > x2 { (idx1, idx2) } else { (idx2, idx1) };

            let west = self.wall_idx(idx_west, Self::WEST);
            let east = self.wall_idx(idx_east, Self::EAST);

            self.walls[west] = WallState::Door(door_id);
            self.walls[east] = WallState::Door(door_id);
        }
    }

    fn format_grid(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        start: usize,
        end: usize,
    ) -> std::fmt::Result {
        write!(f, "+")?;
        for _ in 0..self.width {
            write!(f, "---+")?;
        }
        writeln!(f)?;

        for y in 0..self.height {
            for x in 0..self.width {
                let idx = y * self.width + x;

                let west_wall = self.walls[self.wall_idx(idx, Self::WEST)];
                match west_wall {
                    WallState::Solid => write!(f, "|")?,
                    WallState::Door(id) => write!(f, "D{}", id)?,
                    WallState::Open => write!(f, " ")?,
                }

                let cell_type = self.grid[idx];
                if idx == start {
                    write!(f, " s ")?;
                } else if idx == end {
                    write!(f, " e ")?;
                } else {
                    match cell_type {
                        CellType::Key(id) => write!(f, " k{}", id)?,
                        CellType::Normal => write!(f, "   ")?,
                        CellType::Void => write!(f, " v ")?,
                    }
                }
            }
            writeln!(f, "|")?;

            write!(f, "+")?;
            for x in 0..self.width {
                let idx = y * self.width + x;
                let south_wall = self.walls[self.wall_idx(idx, Self::SOUTH)];
                match south_wall {
                    WallState::Open => write!(f, "   +")?,
                    WallState::Solid => write!(f, "---+")?,
                    WallState::Door(id) => write!(f, "-D{}-+", id)?,
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}
