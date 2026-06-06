use rand::RngExt;

use crate::{generator::MazeGenerator, grid::Shape};

pub struct PrimGenerator;

impl PrimGenerator {}

impl MazeGenerator for PrimGenerator {
    fn generate<S: Shape>(
        &self,
        maze: &mut crate::maze::Maze<S>,
        rng: &mut rand::prelude::ThreadRng,
    ) {
        let mut visited = vec![false; maze.size()];
        let mut frontier = Vec::<usize>::new();

        let start_idx = rng.random_range(0..maze.size());
        maze.set_start(start_idx);

        visited[start_idx] = true;
        for neighbour in maze.get_neighbours(start_idx) {
            if !visited[neighbour] {
                frontier.push(neighbour);
            }
        }

        while !frontier.is_empty() {
            let idx = rng.random_range(0..frontier.len());
            let current = frontier.swap_remove(idx);

            if visited[current] {
                continue;
            }

            visited[current] = true;
            let visited_neighbours: Vec<usize> = maze
                .get_neighbours(current)
                .iter()
                .filter(|&&n| visited[n])
                .cloned()
                .collect();

            if !visited_neighbours.is_empty() {
                let neighbour = visited_neighbours[rng.random_range(0..visited_neighbours.len())];
                maze.remove_wall(current, neighbour);
                for n in maze.get_neighbours(current) {
                    if !visited[n] {
                        frontier.push(n);
                    }
                }
            }
        }
    }
}
