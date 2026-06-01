use std::collections::VecDeque;

use bitflags::bitflags;
use rand::{RngExt, rngs::ThreadRng};

bitflags! {
    #[derive(Clone, Copy)]
    pub struct Walls: u8 {
        const NORTH = 0b00000001;
        const WEST = 0b00000010;
        const EAST = 0b00000100;
        const SOUTH = 0b00001000;
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq)]
enum CellType {
    DOOR,
    KEY,
    #[default]
    NORMAL,
}

#[derive(Clone, Copy)]
struct Cell {
    walls: Walls,
    cell_type: CellType,
}

impl Cell {
    fn new() -> Self {
        Self {
            walls: Walls::NORTH | Walls::SOUTH | Walls::WEST | Walls::EAST,
            cell_type: CellType::default(),
        }
    }
}

struct Maze {
    width: usize,
    height: usize,
    grid: Vec<Cell>,
    start: usize,
    end: usize,
}

impl Maze {
    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            grid: vec![Cell::new(); width * height],
            start: 0,
            end: 0,
        }
    }

    fn generate_prim(&mut self) {
        let mut rng = rand::rng();

        let mut visited = vec![false; self.width * self.height];
        let mut frontier = Vec::<(usize, usize)>::new();

        let start_idx = rng.random_range(0..visited.len());
        self.start = start_idx;
        let start_x = start_idx % self.width;
        let start_y = start_idx / self.width;

        visited[start_idx] = true;
        self.add_unvisited_to_frontier(start_x, start_y, &visited, &mut frontier);

        while !frontier.is_empty() {
            let idx = rng.random_range(0..frontier.len());
            let (x, y) = frontier.swap_remove(idx);

            if visited[y * self.width + x] {
                continue;
            }

            visited[y * self.width + x] = true;
            self.connect_cell_to_neighbour(x, y, &visited, &mut frontier, &mut rng);
        }
        self.create_end_cell();
    }

    fn connect_cell_to_neighbour(
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

            self.remove_separator_between_cells(x1, x2, y1, y2);

            self.add_unvisited_to_frontier(x1, y1, visited, frontier);
        }
    }

    fn add_unvisited_to_frontier(
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

    fn remove_separator_between_cells(&mut self, x1: usize, x2: usize, y1: usize, y2: usize) {
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

    fn create_end_cell(&mut self) {
        let mut visited = vec![false; self.width * self.height];
        let mut queue = VecDeque::new();
        let mut end_idx = self.start;

        queue.push_front(self.start);
        visited[self.start] = true;

        while let Some(current) = queue.pop_back() {
            end_idx = current;

            let x = current % self.width;
            let y = current / self.width;
            let cell = self.grid[current];

            if !cell.walls.contains(Walls::NORTH) && y > 0 {
                let n_neighbour_idx = current - self.width;
                if !visited[n_neighbour_idx] {
                    visited[n_neighbour_idx] = true;
                    queue.push_front(n_neighbour_idx);
                }
            }
            if !cell.walls.contains(Walls::SOUTH) && y < self.height - 1 {
                let s_neighbour_idx = current + self.width;
                if !visited[s_neighbour_idx] {
                    visited[s_neighbour_idx] = true;
                    queue.push_front(s_neighbour_idx);
                }
            }
            if !cell.walls.contains(Walls::WEST) && x > 0 {
                let w_neighbour_idx = current - 1;
                if !visited[w_neighbour_idx] {
                    visited[w_neighbour_idx] = true;
                    queue.push_front(w_neighbour_idx);
                }
            }
            if !cell.walls.contains(Walls::EAST) && x < self.width - 1 {
                let e_neighbour_idx = current + 1;
                if !visited[e_neighbour_idx] {
                    visited[e_neighbour_idx] = true;
                    queue.push_front(e_neighbour_idx);
                }
            }
        }

        self.end = end_idx;
    }

    fn find_path_to_exit(&self) -> Vec<usize> {
        let mut visited = vec![false; self.width * self.height];
        let mut path = Vec::<usize>::new();
        self.dfs(self.start, self.end, &mut path, &mut visited);
        path
    }

    fn dfs(&self, current: usize, end: usize, path: &mut Vec<usize>, visited: &mut [bool]) -> bool {
        if current == end {
            path.push(current);
            return true;
        }
        visited[current] = true;
        path.push(current);

        let x = current % self.width;
        let y = current / self.width;
        let cell = self.grid[current];

        if !cell.walls.contains(Walls::NORTH) && y > 0 {
            let n = current - self.width;
            if !visited[n] && self.dfs(n, end, path, visited) {
                return true;
            }
        }

        if !cell.walls.contains(Walls::SOUTH) && y < self.height - 1 {
            let s = current + self.width;
            if !visited[s] && self.dfs(s, end, path, visited) {
                return true;
            }
        }

        if !cell.walls.contains(Walls::EAST) && x < self.width - 1 {
            let e = current + 1;
            if !visited[e] && self.dfs(e, end, path, visited) {
                return true;
            }
        }

        if !cell.walls.contains(Walls::WEST) && x > 0 {
            let w = current - 1;
            if !visited[w] && self.dfs(w, end, path, visited) {
                return true;
            }
        }
        path.pop();
        false
    }

    fn print_maze(&self) {
        print!("+");
        for _ in 0..self.width {
            print!("---+");
        }
        println!();

        for y in 0..self.height {
            for x in 0..self.width {
                let idx = y * self.width + x;
                let cell = self.grid[idx];

                if cell.walls.contains(Walls::WEST) {
                    print!("|");
                } else {
                    print!(" ");
                }
                let cell_char = if idx == self.start {
                    " s "
                } else if idx == self.end {
                    " e "
                } else {
                    match cell.cell_type {
                        CellType::DOOR => " d ",
                        CellType::KEY => " k ",
                        CellType::NORMAL => "   ",
                    }
                };
                print!("{cell_char}");
            }
            println!("|");

            print!("+");
            for x in 0..self.width {
                let cell = self.grid[y * self.width + x];

                if cell.walls.contains(Walls::SOUTH) {
                    print!("---+");
                } else {
                    print!("   +");
                }
            }
            println!();
        }
    }
}

fn main() {
    let mut maze = Maze::new(10, 15);
    maze.generate_prim();
    maze.print_maze();
    println!("{:?}", maze.find_path_to_exit());
}
