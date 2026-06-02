use std::collections::{HashSet, VecDeque};

use bitflags::bitflags;
use rand::{RngExt, rngs::ThreadRng, seq::IndexedRandom};

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
    Door(u8),
    Key(u8),
    #[default]
    Normal,
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
        self.generate_doors_and_keys(&mut rng);
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

            for neighbour in self.get_neighbours(current) {
                if !visited[neighbour] {
                    visited[neighbour] = true;
                    queue.push_front(neighbour);
                }
            }
        }

        self.end = end_idx;
    }

    fn get_neighbours(&self, current: usize) -> Vec<usize> {
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

        for neighbour in self.get_neighbours(current) {
            if !visited[neighbour] && self.dfs(neighbour, end, path, visited) {
                return true;
            }
        }
        path.pop();
        false
    }

    fn generate_doors_and_keys(&mut self, rng: &mut ThreadRng) {
        let path = self.find_path_to_exit();
        if let Some(doors) = self.place_doors(&path[1..&path.len() - 1], rng) {
            self.place_keys(&path, &doors, rng);
        }
    }

    fn place_doors(&mut self, path: &[usize], rng: &mut ThreadRng) -> Option<Vec<usize>> {
        let mut door_id = 0;
        let max_doors = path.len() / 5;
        if max_doors == 0 {
            return None;
        }

        let number_of_doors = rng.random_range(1..=max_doors);
        let mut doors = Vec::with_capacity(number_of_doors);

        for i in 0..number_of_doors {
            let region_start = i * path.len() / number_of_doors;
            let region_end = ((i + 1) * path.len() / number_of_doors).min(path.len() - 1);

            let door_pos = path[rng.random_range(region_start..region_end)];

            if self.grid[door_pos].cell_type == CellType::Normal {
                self.grid[door_pos].cell_type = CellType::Door(door_id);
                door_id += 1;
                doors.push(door_pos);
            }
        }

        Some(doors)
    }

    fn place_keys(&mut self, path: &[usize], doors: &[usize], rng: &mut ThreadRng) {
        let path_set: HashSet<usize> = path.iter().copied().collect();
        for i in 0..doors.len() {
            let accessible = self.get_accessible_zone(&doors[i..]);

            let mut visited = vec![false; self.height * self.width];
            let mut dead_ends = Vec::<usize>::new();
            let mut queue = VecDeque::new();

            queue.push_back(self.start);
            visited[self.start] = true;

            while let Some(current) = queue.pop_front() {
                for neighbour in self.get_neighbours(current) {
                    if accessible.contains(&neighbour) && !visited[neighbour] {
                        queue.push_back(neighbour);
                        visited[neighbour] = true;
                        if self.is_dead_end(neighbour) && !path_set.contains(&neighbour) {
                            dead_ends.push(neighbour);
                        }
                    }
                }
            }
            let dead_ends = &dead_ends[dead_ends.len() / 2..];
            if let CellType::Door(id) = self.grid[doors[i]].cell_type {
                self.grid[*dead_ends.choose(rng).unwrap()].cell_type = CellType::Key(id);
            }
        }
    }

    fn is_dead_end(&self, idx: usize) -> bool {
        let cell = self.grid[idx];
        [Walls::NORTH, Walls::SOUTH, Walls::EAST, Walls::WEST]
            .iter()
            .filter(|&&w| cell.walls.contains(w))
            .count()
            == 3
    }

    fn get_accessible_zone(&self, locked_doors_positions: &[usize]) -> HashSet<usize> {
        let locked_doors_positions: HashSet<usize> =
            locked_doors_positions.iter().cloned().collect();
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
                        CellType::Door(id) => &format!(" d{id}"),
                        CellType::Key(id) => &format!(" k{id}"),
                        CellType::Normal => "   ",
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
    let mut maze = Maze::new(5, 7);
    maze.generate_prim();
    maze.print_maze();
    println!("{:?}", maze.find_path_to_exit());
}
