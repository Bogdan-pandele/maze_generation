use appcui::graphics::{CharAttribute, Character, Color, LineType, Surface};
use maze_logic::{
    cell::{CellType, WallState},
    grid::shapes::triangle::TriangularGrid,
    maze::Maze,
};

use crate::maze::maze_drawer::{MazeDrawer, get_key_door_color};

fn draw_triangular_cell(
    maze: &Maze<TriangularGrid>,
    surface: &mut Surface,
    row: i32,
    col: i32,
    cell_size: i32,
    offset_x: i32,
    offset_y: i32,
    player_pos: usize,
) {
    let x_left = offset_x + col * cell_size;
    let x_right = x_left + 2 * cell_size;
    let x_middle = x_left + cell_size;

    let y_top = offset_y + cell_size * row;
    let y_bottom = y_top + cell_size;

    let is_upside_down = (row + col) % 2 != 0;

    let end = maze.end();

    let current_idx = row as usize * maze.width() + col as usize;
    let walls = maze.wall_states_for_cell(current_idx);

    let edges = if !is_upside_down {
        [
            (x_left, y_bottom, x_middle, y_top),
            (x_middle, y_top, x_right, y_bottom),
            (x_right, y_bottom, x_left, y_bottom),
        ]
    } else {
        [
            (x_left, y_top, x_middle, y_bottom),
            (x_middle, y_bottom, x_right, y_top),
            (x_right, y_top, x_left, y_top),
        ]
    };

    for (i, wallstate) in walls.iter().enumerate() {
        let (x1, y1, x2, y2) = edges[i];
        match wallstate {
            WallState::Open => {}
            WallState::Solid => {
                surface.draw_line(
                    x1,
                    y1,
                    x2,
                    y2,
                    LineType::Border,
                    CharAttribute::with_fore_color(Color::White),
                );
            }
            WallState::Door(id) => {
                let door_color = get_key_door_color(*id);
                surface.draw_line(
                    x1,
                    y1,
                    x2,
                    y2,
                    LineType::Border,
                    CharAttribute::with_fore_color(door_color),
                );
            }
        }
    }

    let x_center = x_middle;
    let y_center = if is_upside_down {
        y_top + cell_size / 3
    } else {
        y_top + 2 * cell_size / 3
    };

    if current_idx == end {
        surface.write_char(
            x_center,
            y_center,
            Character::with_attributes('🥅', CharAttribute::with_fore_color(Color::Red)),
        );
    } else if let CellType::Key(id) = maze.cell_type(current_idx) {
        let key_color = get_key_door_color(id);
        surface.write_char(
            x_center,
            y_center,
            Character::with_attributes('⚷', CharAttribute::with_fore_color(key_color)),
        );
    } else if current_idx == player_pos {
        surface.write_char(
            x_center,
            y_center,
            Character::with_attributes(
                '🟡',
                CharAttribute::with_color(Color::Yellow, Color::Black),
            ),
        );
    }
}

impl MazeDrawer for Maze<TriangularGrid> {
    fn draw(
        &self,
        surface: &mut appcui::prelude::Surface,
        cell_size: i32,
        offset_x: i32,
        offset_y: i32,
        player_pos: usize,
    ) {
        let width = self.width() as i32;
        let height = self.height() as i32;

        for row in 0..height {
            for col in 0..width {
                draw_triangular_cell(
                    self, surface, row, col, cell_size, offset_x, offset_y, player_pos,
                );
            }
        }
    }
}
