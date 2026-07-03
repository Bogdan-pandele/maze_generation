use rand::rngs::ThreadRng;

use crate::{
    algorithms::find_furthest, generator::MazeGenerator, grid::Shape, maze::Maze,
    obstacles::place_doors_and_keys,
};

mod algorithms;
pub mod cell;
pub mod generator;
pub mod grid;
pub mod maze;
mod obstacles;

pub fn build<S: Shape>(shape: S, algorithm: impl MazeGenerator, use_obstacles: bool) -> Maze<S> {
    let mut rng = ThreadRng::default();

    let mut maze = Maze::new(shape);

    algorithm.generate(&mut maze, &mut rng);

    let end = find_furthest(&maze, maze.start());

    maze.set_end(end);
    if use_obstacles {
        place_doors_and_keys(&mut maze, &mut rng);
    }

    maze
}
