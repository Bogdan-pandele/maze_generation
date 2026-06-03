use rand::rngs::ThreadRng;

use crate::{
    algorithms::find_furthest,
    generator::{MazeGenerator, generation_algorithms::prim::PrimGenerator},
    maze::Maze,
    obstacles::place_doors_and_keys,
};

mod algorithms;
pub mod cell;
pub mod generator;
pub mod maze;
mod obstacles;

pub fn build(width: usize, height: usize, rng: &mut ThreadRng) -> Maze {
    let mut maze = Maze::new(width, height);

    PrimGenerator::generate(&mut maze, rng);

    let end = find_furthest(&maze, maze.start());

    maze.set_end(end);

    place_doors_and_keys(&mut maze, rng);

    maze
}
