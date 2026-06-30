use rand::RngExt;

use crate::generator::MazeGenerator;

pub struct CuttingGenerator;

impl MazeGenerator for CuttingGenerator {
    fn generate<S: crate::grid::Shape>(
        &self,
        maze: &mut crate::maze::Maze<S>,
        rng: &mut rand::prelude::ThreadRng,
    ) {
        let mut v = Vec::new();
        let size = maze.size();
        let width = maze.width();
        let height = maze.height();

        for _ in 0..5 {
            let idx1 = rng.random_range(0..size);
            let r1 = idx1 / width;
            let start_r1 = r1 * width;

            for i in 0..width - 1 {
                maze.remove_wall(start_r1 + i, start_r1 + i + 1);
            }

            let idx2 = rng.random_range(0..size);
            let c2 = idx2 % width;
            for i in 0..height - 1 {
                maze.remove_wall(c2 + i * width, c2 + (i + 1) * width);
            }

            v.push((idx1, idx2));
        }

        let r3 = rng.random_range(0..v.len());
        let (s, _) = v[r3];
        maze.set_start(s);
    }
}
