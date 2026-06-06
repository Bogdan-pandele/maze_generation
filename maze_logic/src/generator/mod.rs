use rand::rngs::ThreadRng;
pub mod generation_algorithms;

use crate::{grid::Shape, maze::Maze};

pub trait MazeGenerator {
    fn generate<S: Shape>(&self, maze: &mut Maze<S>, rng: &mut ThreadRng);
}
