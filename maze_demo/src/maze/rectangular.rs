use appcui::graphics::{CharAttribute, Color, LineType, Surface};
use maze_logic::{
    cell::{CellType, WallState},
    grid::shapes::rectangle::RectangularGrid,
    maze::Maze,
};

use crate::maze::maze_drawer::{MazeDrawer, get_key_door_color};

fn draw_rectangular_cell(
    maze: &Maze<RectangularGrid>,
    surface: &mut Surface,
    row: i32,
    col: i32,
    cell_size: i32,
    offset_x: i32,
    offset_y: i32,
) {
    let x_left = offset_x + col * cell_size * 2;
    let y_top = offset_y + row * cell_size;
    let x_right = x_left + cell_size * 2;
    let y_bottom = y_top + cell_size;

    let start = maze.start();
    let end = maze.end();

    let current_idx = row as usize * maze.shape().width() + col as usize;
    let walls = maze.wall_states_for_cell(current_idx);

    let edges = [
        (x_left, y_top, x_right, y_top),
        (x_left, y_top, x_left, y_bottom),
        (x_left, y_bottom, x_right, y_bottom),
        (x_right, y_bottom, x_right, y_top),
    ];

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

    let x_center = x_left + cell_size;
    let y_center = y_top + cell_size / 2;
    if current_idx == start {
        surface.write_string(
            x_center,
            y_center,
            "S",
            CharAttribute::with_fore_color(Color::DarkGreen),
            false,
        );
    } else if current_idx == end {
        surface.write_string(
            x_center,
            y_center,
            "E",
            CharAttribute::with_fore_color(Color::DarkRed),
            false,
        );
    } else if let CellType::Key(id) = maze.cell_type(current_idx) {
        let key_color = get_key_door_color(id);
        surface.write_string(
            x_center,
            y_center,
            &format!("K{id}"),
            CharAttribute::with_fore_color(key_color),
            false,
        );
    }
}

impl MazeDrawer for Maze<RectangularGrid> {
    fn draw(&self, surface: &mut Surface, cell_size: i32, off_x: i32, off_y: i32) {
        let width = self.shape().width() as i32;
        let height = self.shape().height() as i32;

        for row in 0..height {
            for col in 0..width {
                draw_rectangular_cell(self, surface, row, col, cell_size, off_x, off_y);
            }
        }
    }
}
