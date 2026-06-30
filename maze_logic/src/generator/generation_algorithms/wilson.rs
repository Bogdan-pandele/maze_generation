use rand::{RngExt, rngs::ThreadRng, seq::IndexedRandom};

use crate::generator::MazeGenerator;

pub struct WilsonGenerator;

impl MazeGenerator for WilsonGenerator {
    fn generate<S: crate::grid::Shape>(
        &self,
        maze: &mut crate::maze::Maze<S>,
        rng: &mut ThreadRng,
    ) {
        let size = maze.size();

        let mut visited = vec![false; size];

        let start_idx = rng.random_range(0..size);
        maze.set_start(start_idx);
        visited[start_idx] = true;

        let mut path = vec![usize::MAX; size];

        for i in 0..size {
            if visited[i] {
                continue;
            }

            let mut current = i;

            while !visited[current] {
                let neighbors = maze.get_neighbours(current);

                let next = *neighbors.choose(rng).unwrap();

                path[current] = next;

                current = next;
            }

            current = i;
            while !visited[current] {
                let next = path[current];

                maze.remove_wall(current, next);

                visited[current] = true;

                current = next;
            }
        }
    }
}
