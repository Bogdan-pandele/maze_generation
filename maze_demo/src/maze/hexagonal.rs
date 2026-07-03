use appcui::graphics::{CharAttribute, Character, Color, LineType, Surface};
use maze_logic::{
    cell::{CellType, WallState},
    grid::shapes::hexagon::HexagonalGrid,
    maze::Maze,
};

use crate::maze::maze_drawer::{MazeDrawer, get_key_door_color};

fn draw_hexagonal_cell(
    maze: &Maze<HexagonalGrid>,
    surface: &mut Surface,
    row: i32,
    col: i32,
    cell_size: i32,
    offset_x: i32,
    offset_y: i32,
    player_pos: usize,
) {
    #[cfg(target_os = "windows")]
    let width_ratio = 140;

    #[cfg(not(target_os = "windows"))]
    let width_ratio = 173;

    let h = cell_size * width_ratio / 100;
    let x_left = if row % 2 == 0 {
        offset_x + col * h * 2
    } else {
        offset_x + col * h * 2 + h
    };
    let x_right = x_left + h * 2;
    let x_middle = x_left + h;

    let y0 = offset_y + (row * 3 * cell_size) / 2;
    let y1 = offset_y + (row * 3 * cell_size + cell_size) / 2;
    let y2 = offset_y + (row * 3 * cell_size + 3 * cell_size) / 2;
    let y3 = offset_y + (row * 3 * cell_size + 4 * cell_size) / 2;

    // let y_top = offset_y + row * cell_size * 3 / 2;
    // let y_bottom = y_top + 2 * cell_size;

    let edges = [
        (x_middle, y0, x_right, y1),
        (x_right, y1, x_right, y2),
        (x_right, y2, x_middle, y3),
        (x_middle, y3, x_left, y2),
        (x_left, y2, x_left, y1),
        (x_left, y1, x_middle, y0),
    ];
    let end = maze.end();

    let current_idx = row as usize * maze.width() + col as usize;
    let walls = maze.wall_states_for_cell(current_idx);

    for (i, wallstate) in walls.iter().enumerate() {
        let (x1, y1, x2, y2) = edges[i];
        match wallstate {
            WallState::Open => {}

            WallState::Solid => surface.draw_line(
                x1,
                y1,
                x2,
                y2,
                LineType::Border,
                CharAttribute::with_fore_color(Color::White),
            ),

            WallState::Door(id) => {
                let door_color = get_key_door_color(*id);
                surface.draw_line(
                    x1,
                    y1,
                    x2,
                    y2,
                    LineType::Border,
                    CharAttribute::with_fore_color(door_color),
                )
            }
        }
    }

    let x_center = x_middle;
    let y_center = offset_y + (row * 3 * cell_size + 2 * cell_size) / 2;
    if current_idx == end {
        surface.write_char(
            x_center,
            y_center,
            Character::with_attributes('\u{2605}', CharAttribute::with_fore_color(Color::Red)),
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
                '\u{25CF}',
                CharAttribute::with_color(Color::Yellow, Color::Black),
            ),
        );
    }
}

impl MazeDrawer for Maze<HexagonalGrid> {
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
                draw_hexagonal_cell(
                    self, surface, row, col, cell_size, offset_x, offset_y, player_pos,
                );
            }
        }
    }
}
