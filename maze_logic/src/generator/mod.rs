use rand::rngs::ThreadRng;
pub mod generation_algorithms;

use crate::maze::Maze;

pub trait MazeGenerator {
    fn generate(maze: &mut Maze, rng: &mut ThreadRng);
}
