use std::collections::HashSet;

use appcui::prelude::CustomControl;
use maze_logic::{
    grid::shapes::{hexagon::HexagonalGrid, rectangle::RectangularGrid, triangle::TriangularGrid},
    maze::Maze,
};

use crate::maze::{
    maze_drawer::MazeDrawer,
    maze_game::ActiveMaze::{Hexagonal, Rectangular, Triangular},
};

pub enum GameState {
    Playing,
    Victory,
}
pub enum ActiveMaze {
    Rectangular(Maze<RectangularGrid>),
    Triangular(Maze<TriangularGrid>),
    Hexagonal(Maze<HexagonalGrid>),
}

impl MazeDrawer for ActiveMaze {
    fn draw(&self, surface: &mut Surface, cell_size: i32, off_x: i32, off_y: i32) {
        match self {
            ActiveMaze::Rectangular(m) => m.draw(surface, cell_size, off_x, off_y),
            ActiveMaze::Triangular(m) => m.draw(surface, cell_size, off_x, off_y),
            ActiveMaze::Hexagonal(m) => m.draw(surface, cell_size, off_x, off_y),
        }
    }
}

impl ActiveMaze {
    fn start(&self) -> usize {
        match self {
            Rectangular(m) => m.start(),
            Triangular(m) => m.start(),
            Hexagonal(m) => m.start(),
        }
    }
}

#[CustomControl(overwrite = OnPaint)]
pub struct MazeGame {
    maze: ActiveMaze,
    player_pos: usize,
    owned_keys: HashSet<u8>,
    state: GameState,
    cell_size: i32,
}

impl MazeGame {
    pub fn new(maze: ActiveMaze, cell_size: i32) -> Self {
        let start = maze.start();

        Self {
            base: ControlBase::new(layout!("d:f"), true),
            maze,
            player_pos: start,
            owned_keys: HashSet::new(),
            state: GameState::Playing,
            cell_size,
        }
    }
}

impl OnPaint for MazeGame {
    fn on_paint(&self, surface: &mut Surface, _theme: &Theme) {
        surface.clear(char!("' ',black,black"));
        self.maze.draw(surface, self.cell_size, 3, 3);
    }
}
