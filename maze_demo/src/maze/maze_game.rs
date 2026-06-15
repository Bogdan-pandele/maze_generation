use std::collections::HashSet;

use appcui::prelude::CustomControl;
use maze_logic::{
    cell::CellType,
    grid::shapes::{hexagon::HexagonalGrid, rectangle::RectangularGrid, triangle::TriangularGrid},
    maze::{Direction, Maze},
};

use crate::maze::{
    maze_drawer::MazeDrawer,
    maze_game::ActiveMaze::{Hexagonal, Rectangular, Triangular},
};

#[derive(PartialEq, Eq)]
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
    fn draw(
        &self,
        surface: &mut Surface,
        cell_size: i32,
        off_x: i32,
        off_y: i32,
        player_pos: usize,
    ) {
        match self {
            ActiveMaze::Rectangular(m) => m.draw(surface, cell_size, off_x, off_y, player_pos),
            ActiveMaze::Triangular(m) => m.draw(surface, cell_size, off_x, off_y, player_pos),
            ActiveMaze::Hexagonal(m) => m.draw(surface, cell_size, off_x, off_y, player_pos),
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

    fn end(&self) -> usize {
        match self {
            Rectangular(m) => m.end(),
            Triangular(m) => m.end(),
            Hexagonal(m) => m.end(),
        }
    }

    fn next_neighbour(
        &self,
        current: usize,
        direction: Direction,
        keys: &HashSet<u8>,
    ) -> Option<usize> {
        match self {
            Rectangular(m) => m.neighbour_in_direction(current, direction, keys),
            Triangular(m) => m.neighbour_in_direction(current, direction, keys),
            Hexagonal(m) => m.neighbour_in_direction(current, direction, keys),
        }
    }

    fn cell_type(&self, current: usize) -> CellType {
        match self {
            Rectangular(m) => m.cell_type(current),
            Triangular(m) => m.cell_type(current),
            Hexagonal(m) => m.cell_type(current),
        }
    }

    fn set_cell_type(&mut self, current: usize, cell_type: CellType) {
        match self {
            Rectangular(m) => m.set_cell_type(current, cell_type),
            Triangular(m) => m.set_cell_type(current, cell_type),
            Hexagonal(m) => m.set_cell_type(current, cell_type),
        }
    }

    fn open_door(&mut self, idx1: usize, idx2: usize) {
        match self {
            Rectangular(m) => m.open_door(idx1, idx2),
            Triangular(m) => m.open_door(idx1, idx2),
            Hexagonal(m) => m.open_door(idx1, idx2),
        }
    }
}

#[CustomControl(overwrite = OnPaint+OnKeyPressed)]
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

    fn move_player(&mut self, direction: Direction) {
        if self.player_pos == self.maze.end() {
            self.state = GameState::Victory;
            return;
        }

        if let Some(neighbour) =
            self.maze
                .next_neighbour(self.player_pos, direction, &self.owned_keys)
        {
            let old_pos = self.player_pos;
            self.player_pos = neighbour;
            if let CellType::Key(id) = self.maze.cell_type(self.player_pos) {
                self.owned_keys.insert(id);
                self.maze.set_cell_type(self.player_pos, CellType::Normal);
            }

            self.maze.open_door(old_pos, self.player_pos);
        }
    }

    fn paint_victory_message(&self, surface: &mut Surface) {
        let size = surface.size();

        let w: i32 = 40;
        let h: i32 = 6;

        let x = (size.width as i32 - w) / 2;
        let y = (size.height as i32 - h) / 2;

        let r = Rect::with_size(x, y, w as u16, h as u16);

        surface.fill_rect(
            r,
            Character::new(' ', Color::White, Color::Aqua, CharFlags::None),
        );

        surface.write_string(
            x + w / 2 - ("M A Z E   C O M P L E T E D !!".len() / 2) as i32,
            y,
            "M A Z E   C O M P L E T E D !!",
            charattr!("white,aqua"),
            false,
        );

        surface.draw_horizontal_line_with_size(
            x + 1,
            y + 1,
            (w - 2) as u32,
            LineType::Single,
            charattr!("gray,aqua"),
        );

        surface.draw_horizontal_line_with_size(
            x + 1,
            y + 4,
            (w - 2) as u32,
            LineType::Single,
            charattr!("gray,aqua"),
        );
    }
}

impl OnPaint for MazeGame {
    fn on_paint(&self, surface: &mut Surface, _theme: &Theme) {
        surface.clear(char!("' ',black,black"));

        match self.state {
            GameState::Playing => self
                .maze
                .draw(surface, self.cell_size, 3, 3, self.player_pos),
            GameState::Victory => {
                self.paint_victory_message(surface);
            }
        }
    }
}

impl OnKeyPressed for MazeGame {
    fn on_key_pressed(&mut self, key: Key, _character: char) -> EventProcessStatus {
        if self.state == GameState::Victory {
            return EventProcessStatus::Ignored;
        }

        match key.code {
            KeyCode::W => {
                self.move_player(Direction::Top);
                EventProcessStatus::Processed
            }

            KeyCode::S => {
                self.move_player(Direction::Bottom);
                EventProcessStatus::Processed
            }

            KeyCode::A => {
                self.move_player(Direction::Left);
                EventProcessStatus::Processed
            }

            KeyCode::D => {
                self.move_player(Direction::Right);
                EventProcessStatus::Processed
            }

            KeyCode::Q => {
                self.move_player(Direction::TopLeft);
                EventProcessStatus::Processed
            }

            KeyCode::Z => {
                self.move_player(Direction::BottomLeft);
                EventProcessStatus::Processed
            }

            KeyCode::E => {
                self.move_player(Direction::TopRight);
                EventProcessStatus::Processed
            }

            KeyCode::C => {
                self.move_player(Direction::BottomRight);
                EventProcessStatus::Processed
            }

            _ => EventProcessStatus::Ignored,
        }
    }
}
