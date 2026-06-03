use rand::RngExt;

use crate::generator::MazeGenerator;

pub struct PrimGenerator;

impl MazeGenerator for PrimGenerator {
    fn generate(maze: &mut crate::maze::Maze, rng: &mut rand::prelude::ThreadRng) {
        let mut visited = vec![false; maze.width() * maze.height()];
        let mut frontier = Vec::<(usize, usize)>::new();

        let start_idx = rng.random_range(0..visited.len());
        maze.set_start(start_idx);
        let start_x = start_idx % maze.width();
        let start_y = start_idx / maze.width();

        visited[start_idx] = true;
        maze.add_unvisited_to_frontier(start_x, start_y, &visited, &mut frontier);

        while !frontier.is_empty() {
            let idx = rng.random_range(0..frontier.len());
            let (x, y) = frontier.swap_remove(idx);

            if visited[y * maze.width() + x] {
                continue;
            }

            visited[y * maze.width() + x] = true;
            maze.connect_cell_to_neighbour(x, y, &visited, &mut frontier, rng);
        }
    }
}
