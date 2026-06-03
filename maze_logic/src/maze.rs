use core::fmt;
use std::collections::{HashSet, VecDeque};

use rand::{RngExt, rngs::ThreadRng};

use crate::cell::{Cell, CellType, Walls};

pub struct Maze {
    width: usize,
    height: usize,
    grid: Vec<Cell>,
    start: usize,
    end: usize,
}

impl Maze {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            grid: vec![Cell::new(); width * height],
            start: 0,
            end: 0,
        }
    }
    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn start(&self) -> usize {
        self.start
    }

    pub fn end(&self) -> usize {
        self.end
    }

    pub fn cell(&self, index: usize) -> &Cell {
        &self.grid[index]
    }

    pub fn set_cell_type(&mut self, idx: usize, cell_type: CellType) {
        self.grid[idx].cell_type = cell_type;
    }

    pub fn set_start(&mut self, start: usize) {
        self.start = start;
    }

    pub fn set_end(&mut self, end: usize) {
        self.end = end;
    }

    pub fn get_neighbours(&self, current: usize) -> Vec<usize> {
        let mut neighbours = Vec::new();
        let x = current % self.width;
        let y = current / self.width;
        let cell = self.grid[current];

        if !cell.walls.contains(Walls::NORTH) && y > 0 {
            neighbours.push(current - self.width);
        }

        if !cell.walls.contains(Walls::SOUTH) && y < self.height - 1 {
            neighbours.push(current + self.width);
        }

        if !cell.walls.contains(Walls::WEST) && x > 0 {
            neighbours.push(current - 1);
        }

        if !cell.walls.contains(Walls::EAST) && x < self.width - 1 {
            neighbours.push(current + 1);
        }

        neighbours
    }

    pub fn is_dead_end(&self, idx: usize) -> bool {
        let cell = self.grid[idx];
        [Walls::NORTH, Walls::SOUTH, Walls::EAST, Walls::WEST]
            .iter()
            .filter(|&&w| cell.walls.contains(w))
            .count()
            == 3
    }

    pub fn get_accessible_zone(&self, locked: &[usize]) -> HashSet<usize> {
        let locked_doors_positions: HashSet<usize> = locked.iter().cloned().collect();
        let mut accessible_zone = HashSet::new();
        let mut visited = vec![false; self.width * self.height];

        let mut queue = VecDeque::new();

        queue.push_back(self.start);
        visited[self.start] = true;

        while let Some(current) = queue.pop_front() {
            accessible_zone.insert(current);

            for neighbour in self.get_neighbours(current) {
                if !visited[neighbour] && !locked_doors_positions.contains(&neighbour) {
                    queue.push_back(neighbour);
                    visited[neighbour] = true;
                }
            }
        }

        accessible_zone
    }

    pub(crate) fn connect_cell_to_neighbour(
        &mut self,
        x1: usize,
        y1: usize,
        visited: &[bool],
        frontier: &mut Vec<(usize, usize)>,
        rng: &mut ThreadRng,
    ) {
        let mut visited_neighbours = Vec::new();

        if y1 > 0 && visited[(y1 - 1) * self.width + x1] {
            visited_neighbours.push((x1, y1 - 1));
        }

        if y1 < self.height - 1 && visited[(y1 + 1) * self.width + x1] {
            visited_neighbours.push((x1, y1 + 1));
        }

        if x1 > 0 && visited[y1 * self.width + x1 - 1] {
            visited_neighbours.push((x1 - 1, y1));
        }

        if x1 < self.width - 1 && visited[y1 * self.width + x1 + 1] {
            visited_neighbours.push((x1 + 1, y1));
        }

        if !visited_neighbours.is_empty() {
            let neighbour_idx = rng.random_range(0..visited_neighbours.len());
            let (x2, y2) = visited_neighbours[neighbour_idx];

            self.remove_separator_between_cells(x1, y1, x2, y2);

            self.add_unvisited_to_frontier(x1, y1, visited, frontier);
        }
    }

    pub(crate) fn add_unvisited_to_frontier(
        &self,
        current_x: usize,
        current_y: usize,
        visited: &[bool],
        frontier: &mut Vec<(usize, usize)>,
    ) {
        if current_y > 0 && !visited[(current_y - 1) * self.width + current_x] {
            frontier.push((current_x, current_y - 1));
        }

        if current_y < self.height - 1 && !visited[(current_y + 1) * self.width + current_x] {
            frontier.push((current_x, current_y + 1));
        }

        if current_x > 0 && !visited[current_y * self.width + (current_x - 1)] {
            frontier.push((current_x - 1, current_y));
        }

        if current_x < self.width - 1 && !visited[current_y * self.width + (current_x + 1)] {
            frontier.push((current_x + 1, current_y));
        }
    }

    fn remove_separator_between_cells(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        let idx1 = y1 * self.width + x1;
        let idx2 = y2 * self.width + x2;

        if x1 == x2 {
            if y1 > y2 {
                self.grid[idx1].walls ^= Walls::NORTH;
                self.grid[idx2].walls ^= Walls::SOUTH;
            } else {
                self.grid[idx1].walls ^= Walls::SOUTH;
                self.grid[idx2].walls ^= Walls::NORTH;
            }
        } else if y1 == y2 {
            if x1 > x2 {
                self.grid[idx1].walls ^= Walls::WEST;
                self.grid[idx2].walls ^= Walls::EAST;
            } else {
                self.grid[idx1].walls ^= Walls::EAST;
                self.grid[idx2].walls ^= Walls::WEST;
            }
        }
    }
}

impl fmt::Display for Maze {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "+")?;
        for _ in 0..self.width {
            write!(f, "---+")?;
        }
        writeln!(f)?;

        for y in 0..self.height() {
            for x in 0..self.width {
                let idx = y * self.width + x;
                let cell = self.grid[idx];

                if cell.walls.contains(Walls::WEST) {
                    write!(f, "|")?;
                } else {
                    write!(f, " ")?;
                }
                let cell_char = if idx == self.start {
                    " s "
                } else if idx == self.end {
                    " e "
                } else {
                    match cell.cell_type {
                        CellType::Door(id) => &format!(" d{id}"),
                        CellType::Key(id) => &format!(" k{id}"),
                        CellType::Normal => "   ",
                    }
                };
                write!(f, "{cell_char}")?;
            }
            writeln!(f, "|")?;

            write!(f, "+")?;
            for x in 0..self.width {
                let cell = self.grid[y * self.width + x];

                if cell.walls.contains(Walls::SOUTH) {
                    write!(f, "---+")?;
                } else {
                    write!(f, "   +")?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}
